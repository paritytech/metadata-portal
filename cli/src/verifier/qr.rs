use std::path::Path;

use anyhow::{anyhow, bail, ensure, Result};
use definitions::error::TransferContent;
use definitions::helpers::multisigner_to_public;
use definitions::metadata::MetaValues;
use definitions::network_specs::{Verifier, VerifierValue};
use definitions::qr_transfers::ContentLoadMeta;
use log::info;
use transaction_parsing::check_signature::pass_crypto;

use crate::common::camera::read_qr_file;
use crate::common::path::{ContentType, QrFileName, QrPath};
use crate::qrs::qrs_in_dir;

pub(crate) fn validate_signed_qrs(dir: impl AsRef<Path>, public_key: &str) -> Result<()> {
    let all_qrs = qrs_in_dir(&dir)?;
    // Quick check that latest files are signed
    for qr in &all_qrs {
        ensure!(qr.file_name.is_signed, "{} is not signed", qr.file_name);
    }

    for qr in &all_qrs {
        if let ContentType::Metadata(_) = qr.file_name.content_type {
            let f_name = &qr.file_name;
            match validate_metadata_qr(qr, public_key) {
                Ok(_) => info!("🎉 {} is verified!", f_name),
                Err(e) => bail!("failed to verify {}: {}", f_name, e),
            }
        }
    }
    Ok(())
}

fn validate_metadata_qr(qr_path: &QrPath, public_key: &str) -> Result<()> {
    ensure!(
        qr_path.file_name.is_signed,
        "{} is not signed",
        qr_path.file_name
    );

    let data_hex = read_qr_file(&qr_path.to_path_buf())?;
    let signed =
        pass_crypto(&data_hex, TransferContent::LoadMeta).map_err(|e| anyhow!("{:?}", e))?;

    verify_signature(&signed.verifier, public_key)?;

    let (meta, _) = ContentLoadMeta::from_slice(&signed.message)
        .meta_genhash()
        .map_err(|e| anyhow!("{:?}", e))?;
    let meta_values = MetaValues::from_slice_metadata(&meta).map_err(|e| anyhow!("{:?}", e))?;

    verify_filename(&meta_values, &qr_path.file_name)?;
    Ok(())
}

fn verify_signature(verifier: &Verifier, public_key: &str) -> Result<()> {
    let discovered_pub_key = match &verifier.v {
        Some(VerifierValue::Standard { m }) => hex::encode(multisigner_to_public(m)),
        _ => bail!("unable to get verifier key from qr file: {:?}", verifier),
    };
    ensure!(
        discovered_pub_key == public_key,
        "public key mismatch! Expected {}, got {}",
        public_key,
        discovered_pub_key
    );
    Ok(())
}

fn verify_filename(meta_values: &MetaValues, actual_qr_name: &QrFileName) -> Result<()> {
    let expected = QrFileName::new(
        &meta_values.name.to_lowercase(),
        ContentType::Metadata(meta_values.version),
        true,
    );
    ensure!(
        actual_qr_name.chain.ends_with(&expected.chain),
        "filename mismatch! Expected {}, got {}",
        expected,
        actual_qr_name
    );
    Ok(())
}
