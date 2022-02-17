use std::path::{Path};
use anyhow::{anyhow, bail, ensure};

use definitions::error::{Signer, TransferContent};
use definitions::helpers::multisigner_to_public;
use definitions::metadata::MetaValues;
use definitions::network_specs::{Verifier, VerifierValue};
use definitions::qr_transfers::ContentLoadMeta;
use transaction_parsing::check_signature::pass_crypto;
use qr_lib::camera::read_qr_movie;
use qr_lib::path::{QrFileName, QrPath};
use qr_lib::read::{latest_qr_per_chain, read_qr_dir};


pub fn validate_signed_qrs(folder: impl AsRef<Path>, public_key: &str) -> anyhow::Result<()> {
    // Quick check that latest files are signed
    for qr_path in latest_qr_per_chain(&folder)?.values() {
        ensure!(qr_path.file_name.is_signed, "{} is not signed", qr_path.file_name);
    }

    for qr_path in read_qr_dir(folder)? {
        let f_name = &qr_path.file_name;
        match validate_qr(&qr_path, public_key) {
            Ok(_) => println!("🎉 {} is verified!", f_name),
            Err(e) => bail!("failed to verify {}: {}", f_name, e),
        }
    }
    Ok(())
}


fn validate_qr(qr_path: &QrPath, public_key: &str) -> anyhow::Result<()> {
    ensure!(qr_path.file_name.is_signed, "{} is not signed", qr_path.file_name);

    let data_hex = read_qr_movie(&qr_path.to_path_buf())?;
    let signed = pass_crypto(&data_hex, TransferContent::LoadMeta).map_err(|e| anyhow!("{:?}", e))?;

    verify_signature(&signed.verifier, public_key)?;

    let (meta, _) = ContentLoadMeta::from_vec(&signed.message).meta_genhash::<Signer>().map_err(|e| anyhow!("{:?}", e))?;
    let meta_values = MetaValues::from_vec_metadata(&meta).map_err(|e| anyhow!("{:?}", e))?;

    verify_filename(&meta_values, &qr_path.file_name)?;
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
    let expected = QrFileName::new(&meta_values.name, meta_values.version, true);
    ensure!(actual_qr_name == &expected, "filename mismatch! Expected {}, got {}", expected, actual_qr_name);
    Ok(())
}
