mod prompt;

use anyhow::bail;
use std::process::Command;

use definitions::crypto::SufficientCrypto;
use definitions::error::TransferContent;
use parity_scale_codec::Decode;

use crate::config::AppConfig;
use crate::lib::camera::read_qr_file;
use crate::lib::path::{ContentType, QrPath};
use generate_message::make_message::make_message;
use generate_message::parser::{Crypto, Goal, Make, Msg};
use qr_reader_pc::{run_with_camera, CameraSettings};
use transaction_parsing::check_signature::pass_crypto;

use crate::lib::read::{all_qrs_in_dir, hex_to_bytes};
use crate::signer::prompt::{select_file, want_to_continue};

pub fn sign(config: AppConfig) -> anyhow::Result<()> {
    let mut files_to_sign: Vec<QrPath> = all_qrs_in_dir(config.qr_dir)?
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
    make_message(make).map_err(anyhow::Error::msg)?;
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
