mod prompt;

use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::process::Command;

use anyhow::{anyhow, bail};
use definitions::crypto::Encryption;
use definitions::error::TransferContent;
use generate_message::full_run;
use generate_message::parser::{
    Command as SignerCommand, Goal, Make, Msg, Signature, Sufficient, Verifier,
};
use qr_reader_pc::{run_with_camera, CameraSettings};
use transaction_parsing::check_signature::pass_crypto;

use crate::common::camera::read_qr_file;
use crate::common::path::{ContentType, QrPath};
use crate::config::AppConfig;
use crate::ethereum::is_ethereum;
use crate::file::files_to_keep;
use crate::signer::prompt::{select_file, want_to_continue};
use crate::source::{read_png_source, save_source_info};

pub(crate) fn sign(config: AppConfig) -> anyhow::Result<()> {
    let mut files_to_sign: Vec<QrPath> = files_to_keep(&config)?
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

    sign_qr(qr_path, signature)?;
    println!("🎉 Signed!");
    Ok(())
}

fn sign_qr(unsigned_qr: &QrPath, signature: String) -> anyhow::Result<QrPath> {
    let mut signed_qr = unsigned_qr.clone();
    signed_qr.file_name.is_signed = true;

    let raw_read = read_qr_file(&unsigned_qr.to_path_buf())?;
    let transfer_content = match unsigned_qr.file_name.content_type {
        ContentType::Metadata(_) => TransferContent::LoadMeta,
        ContentType::Specs => TransferContent::AddSpecs,
    };
    let passed_crypto = pass_crypto(&raw_read, transfer_content).map_err(|e| anyhow!("{:?}", e))?;

    let msg_type = match unsigned_qr.file_name.content_type {
        ContentType::Metadata(_) => Msg::LoadMetadata,
        ContentType::Specs => Msg::AddSpecs,
    };
    let tmp_dir = tempfile::tempdir()?;
    let content_file = tmp_dir.path().join("content");
    let mut f = File::create(&content_file)?;
    f.write_all(passed_crypto.message.deref())?;

    let signing_algorithm = match is_ethereum(&signed_qr.file_name.chain) {
        true => Encryption::Ethereum,
        false => Encryption::Sr25519,
    };
    let make = Make {
        goal: Goal::Qr,
        verifier: Verifier {
            verifier_alice: None,
            verifier_hex: None,
            verifier_file: None,
        },
        signature: Signature {
            signature_hex: None,
            signature_file: None,
        },
        sufficient: Sufficient {
            sufficient_hex: Some(signature),
            sufficient_file: None,
        },
        msg: msg_type,
        name: Some(signed_qr.to_path_buf()),
        files_dir: signed_qr.dir.clone(),
        payload: content_file,
        export_dir: signed_qr.dir.clone(),
        crypto: Some(signing_algorithm),
    };
    println!("⚙ generating {signed_qr}...");
    full_run(SignerCommand::Make(make)).map_err(|e| anyhow!("{:?}", e))?;
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
