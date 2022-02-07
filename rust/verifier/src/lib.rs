use std::convert::TryFrom;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{anyhow, bail, ensure};

use definitions::error::{Signer, TransferContent};
use definitions::helpers::multisigner_to_public;
use definitions::metadata::MetaValues;
use definitions::network_specs::{Verifier, VerifierValue};
use definitions::qr_transfers::ContentLoadMeta;
use transaction_parsing::check_signature::pass_crypto;
use qr_lib::filename::QrFileName;
use qr_lib::fs::latest_qr_per_chain;

mod camera;
    use crate::camera::read_qr_movie;


pub fn validate_signed_qrs(folder: impl AsRef<Path>, public_key: &str) -> anyhow::Result<()> {
    // Quick check that latest files are signed
    for qr_file in latest_qr_per_chain(&folder)?.values() {
        ensure!(qr_file.is_signed, "{} is not signed", qr_file);
    }

    for file in fs::read_dir(folder)? {
        let path = file?.path();
        let f_name = path.file_name().unwrap().to_str().unwrap();
        match validate_qr(&path, public_key) {
            Ok(_) => println!("🎉 {} is verified!", f_name),
            Err(e) => bail!("failed to verify {}: {}", f_name, e),
        }
    }
    Ok(())
}


fn validate_qr(file_path: &PathBuf, public_key: &str) -> anyhow::Result<()> {
    let file_name = QrFileName::try_from(file_path)?;
    ensure!(file_name.is_signed, "{} is not signed", file_path.to_str().unwrap());

    let data_hex = read_qr_movie(file_path)?;
    let signed = pass_crypto(&data_hex, TransferContent::LoadMeta).map_err(|e| anyhow!("{:?}", e))?;

    verify_signature(&signed.verifier, public_key)?;

    let (meta, _) = ContentLoadMeta::from_vec(&signed.message).meta_genhash::<Signer>().map_err(|e| anyhow!("{:?}", e))?;
    let meta_values = MetaValues::from_vec_metadata(&meta).map_err(|e| anyhow!("{:?}", e))?;

    verify_filename(&meta_values, &file_name)?;
    Ok(())
}


fn verify_signature(verifier: &Verifier, public_key: &str) -> anyhow::Result<()> {
    let discovered_pub_key = match &verifier.0 {
        Some(VerifierValue::Standard(m)) => hex::encode(multisigner_to_public(m)),
        _ => bail!("unable to get verifier key from qr file: {:?}", verifier)
    };
    ensure!(discovered_pub_key == public_key, "public key mismatch! Expected {}, got {}", public_key, discovered_pub_key);
    Ok(())
}

fn verify_filename(meta_values: &MetaValues, actual_qr_name: &QrFileName) -> anyhow::Result<()> {
    let expected = QrFileName{kind: "metadata".to_string(), chain: meta_values.name.clone(), version: meta_values.version, is_signed: true};
    ensure!(actual_qr_name == &expected, "filename mismatch! Expected {}, got {}", expected, actual_qr_name);
    Ok(())
}
