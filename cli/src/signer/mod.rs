pub mod prompt;

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

pub(crate) fn sign_qr(unsigned_qr: &QrPath, signature: &str) -> anyhow::Result<QrPath> {
    println!("signature is {}", signature);
    //auto signing signature
    //0116d5a6266345874d8f5b7f88a6619711b2829b52b2865826b1ecefb62beef34fe2d949a2f9a282e73c484e0b9b690c25fa13e82ce1aa89b3f792ba8925f57e77340bc58abf15f6852de8de52e148c861c7a841beb8994dbba5ac2d181e1dd385

    //maual signing signature
    //0116d5a6266345874d8f5b7f88a6619711b2829b52b2865826b1ecefb62beef34f0c7e29790a21be2bd687e3af88e74ac031ad311e3ed8bc58c52e811db81d4e0c6abd134ef98aeb4b1785e851e16733073530d963ff0e3ca4a18d16c01d1e2e8f
    let signature = hex_to_bytes(signature)?;
    let sufficient_crypto = <SufficientCrypto>::decode(&mut &signature[..])?;
    println!("{:?}", sufficient_crypto);
    //auto signing
    //Sr25519 { public: 16d5a6266345874d8f5b7f88a6619711b2829b52b2865826b1ecefb62beef34f (5CaeNrds...), signature: 666f15329bad95b4b4a474265618ce275bcc8337ff07f8d7f6b5335a52e3ac377b51fb7836aafabbec39f9fa3f4b9dd1323fc269e5fea4b6259f53f436be5e88 }
    //manual signing
    //Sr25519 { public: 16d5a6266345874d8f5b7f88a6619711b2829b52b2865826b1ecefb62beef34f (5CaeNrds...), signature: beda51f98c018d3249f799998dba075e45a636d6e41fee84b9ae197a30c6e6731e85637797a28b7ed25973dd531bf9503c64848302f9b6a2fb49f490bd9da68e }
    println!("1");
    let mut signed_qr = unsigned_qr.clone();
    signed_qr.file_name.is_signed = true;

    let raw_read = read_qr_file(&unsigned_qr.to_path_buf())?;
    let transfer_content = match unsigned_qr.file_name.content_type {
        ContentType::Metadata(_) => TransferContent::LoadMeta,
        ContentType::Specs => TransferContent::AddSpecs,
    };
    println!("2");
    let passed_crypto = pass_crypto(&raw_read, transfer_content)
        .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;

    let msg_type = match unsigned_qr.file_name.content_type {
        ContentType::Metadata(_) => Msg::LoadMetadata,
        ContentType::Specs => Msg::AddSpecs,
    };
    println!("3");
    let make = Make {
        goal: Goal::Qr,
        crypto: Crypto::Sufficient(sufficient_crypto),
        msg: msg_type(passed_crypto.message),
        name: Some(signed_qr.to_string()),
    };
    println!("4");
    println!("⚙ generating {}...", signed_qr);
    full_run(SignerCommand::Make(make)).map_err(anyhow::Error::msg)?;
    // Preserve png source information
    println!("5");
    if let Some(png_source) = read_png_source(&unsigned_qr.to_path_buf())? {
        save_source_info(&signed_qr.to_path_buf(), &png_source)?;
    };
    println!("6");
    Ok(signed_qr)
}

fn open_in_browser(file: &QrPath) -> anyhow::Result<()> {
    let cmd = format!("python3 -mwebbrowser file://{}", file);
    let output = Command::new("sh").arg("-c").arg(cmd).output()?;
    if !output.status.success() {
        bail!(
            "error showing QR code: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    }
    Ok(())
}
