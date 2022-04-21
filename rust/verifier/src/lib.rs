use anyhow::{anyhow, bail, ensure};
use std::path::Path;

use definitions::error::{Signer, TransferContent};
use definitions::helpers::multisigner_to_public;
use definitions::metadata::MetaValues;
use definitions::network_specs::{Verifier, VerifierValue};
use definitions::qr_transfers::ContentLoadMeta;
use qr_lib::camera::read_qr_file;
use qr_lib::path::{ContentType, QrFileName, QrPath};
use qr_lib::read::{all_qrs_in_dir, metadata_qr_in_dir};
use transaction_parsing::check_signature::pass_crypto;

pub fn validate_signed_qrs(dir: impl AsRef<Path>, public_key: &str) -> anyhow::Result<()> {
    // Quick check that latest files are signed
    for qr_path in all_qrs_in_dir(&dir)? {
        ensure!(
            qr_path.file_name.is_signed,
            "{} is not signed",
            qr_path.file_name
        );
    }

    for (qr_path, _) in metadata_qr_in_dir(&dir)?.values() {
        let f_name = &qr_path.file_name;
        match validate_metadata_qr(qr_path, public_key) {
            Ok(_) => println!("🎉 {} is verified!", f_name),
            Err(e) => bail!("failed to verify {}: {}", f_name, e),
        }
    }
    Ok(())
}

fn validate_metadata_qr(qr_path: &QrPath, public_key: &str) -> anyhow::Result<()> {
    ensure!(
        qr_path.file_name.is_signed,
        "{} is not signed",
        qr_path.file_name
    );

    let data_hex = read_qr_file(&qr_path.to_path_buf())?;
    let signed =
        pass_crypto(&data_hex, TransferContent::LoadMeta).map_err(|e| anyhow!("{:?}", e))?;

    verify_signature(&signed.verifier, public_key)?;

    let (meta, _) = ContentLoadMeta::from_vec(&signed.message)
        .meta_genhash::<Signer>()
        .map_err(|e| anyhow!("{:?}", e))?;
    let meta_values = MetaValues::from_vec_metadata(&meta).map_err(|e| anyhow!("{:?}", e))?;

    verify_filename(&meta_values, &qr_path.file_name)?;
    Ok(())
}

fn verify_signature(verifier: &Verifier, public_key: &str) -> anyhow::Result<()> {
    let discovered_pub_key = match &verifier.0 {
        Some(VerifierValue::Standard(m)) => hex::encode(multisigner_to_public(m)),
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

fn verify_filename(meta_values: &MetaValues, actual_qr_name: &QrFileName) -> anyhow::Result<()> {
    let expected = QrFileName::new(
        &meta_values.name,
        ContentType::Metadata(meta_values.version),
        true,
    );
    ensure!(
        actual_qr_name == &expected,
        "filename mismatch! Expected {}, got {}",
        expected,
        actual_qr_name
    );
    Ok(())
}
