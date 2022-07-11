mod prompt;

use std::process::Command;

use anyhow::bail;
use definitions::crypto::SufficientCrypto;
use definitions::error::TransferContent;
use generate_message::full_run;
use generate_message::parser::{Command as SignerCommand, Crypto, Goal, Make, Msg};
use parity_scale_codec::Decode;
use qr_reader_pc::{run_with_camera, CameraSettings};
use transaction_parsing::check_signature::pass_crypto;

use crate::config::AppConfig;
use crate::lib::camera::read_qr_file;
use crate::lib::path::{ContentType, QrPath};
use crate::lib::string::hex_to_bytes;
use crate::qrs::qrs_in_dir;
use crate::signer::prompt::{select_file, want_to_continue};
use crate::source::{read_png_source, save_source_info};

pub(crate) fn sign(config: AppConfig) -> anyhow::Result<()> {
    let mut files_to_sign: Vec<QrPath> = qrs_in_dir(config.qr_dir)?
        .into_iter()
        .filter(|qr| !qr.file_name.is_signed)
        .collect();

    match files_to_sign.len() {
        0 => println!("✔ Nothing to sign"),
        _ => {
            while !files_to_sign.is_empty() {
                let i = select_file(&files_to_sign);
                run_for_file(&files_to_sign.swap_remove(i))?
            }
        }
    }
    Ok(())
}

fn run_for_file(qr_path: &QrPath) -> anyhow::Result<()> {
    open_in_browser(qr_path)?;

    if !want_to_continue() {
        println!("Skipping");
        return Ok(());
    }

    let signature = match run_with_camera(CameraSettings { index: Some(0) }) {
        Ok(line) => line,
        Err(e) => bail!("QR reading error. {}", e),
    };

    sign_qr(qr_path, &signature)?;
    println!("🎉 Signed!");
    Ok(())
}

fn sign_qr(unsigned_qr: &QrPath, signature: &str) -> anyhow::Result<QrPath> {
    let signature = hex_to_bytes(signature)?;
    let sufficient_crypto = <SufficientCrypto>::decode(&mut &signature[..])?;

    let mut signed_qr = unsigned_qr.clone();
    signed_qr.file_name.is_signed = true;

    let raw_read = read_qr_file(&unsigned_qr.to_path_buf())?;
    let transfer_content = match unsigned_qr.file_name.content_type {
        ContentType::Metadata(_) => TransferContent::LoadMeta,
        ContentType::Specs => TransferContent::AddSpecs,
    };
    let passed_crypto = pass_crypto(&raw_read, transfer_content)
        .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;

    let msg_type = match unsigned_qr.file_name.content_type {
        ContentType::Metadata(_) => Msg::LoadMetadata,
        ContentType::Specs => Msg::AddSpecs,
    };
    let make = Make {
        goal: Goal::Qr,
        crypto: Crypto::Sufficient(sufficient_crypto),
        msg: msg_type(passed_crypto.message),
        name: Some(signed_qr.to_string()),
    };
    println!("⚙ generating {}...", signed_qr);
    full_run(SignerCommand::Make(make)).map_err(anyhow::Error::msg)?;
    // Preserve png source information
    if let Some(png_source) = read_png_source(&unsigned_qr.to_path_buf())? {
        save_source_info(&signed_qr.to_path_buf(), &png_source)?;
    };
    Ok(signed_qr)
}

fn open_in_browser(file: &QrPath) -> anyhow::Result<()> {
    let cmd = format!(
        "python3 -mwebbrowser file://{}",
        percent_encode(file.to_string())
    );
    let output = Command::new("sh").arg("-c").arg(cmd).output()?;
    if !output.status.success() {
        bail!(
            "error showing QR code: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    }
    Ok(())
}

fn percent_encode<T: AsRef<str>>(input: T) -> String {
    input.as_ref().replace(' ', "%20")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_percent_encode() {
        assert_eq!(percent_encode("foo <bar>"), "foo%20<bar>");
    }
}
