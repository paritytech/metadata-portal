use std::collections::HashMap;
use std::path::Path;

use anyhow::{anyhow, bail, ensure, Result};
use definitions::crypto::Encryption;
use definitions::error::TransferContent;
use definitions::helpers::{multisigner_to_encryption, multisigner_to_public};
use definitions::metadata::MetaValues;
use definitions::network_specs::Verifier as NetworkVerifier;
use definitions::network_specs::VerifierValue;
use definitions::qr_transfers::ContentLoadMeta;
use log::info;
use transaction_parsing::check_signature::pass_crypto;

use crate::config::Verifier;
use crate::qrs::qrs_in_dir;
use crate::utils::camera::read_qr_file;
use crate::utils::path::{ContentType, QrFileName, QrPath};
use crate::AppConfig;

pub(crate) fn validate_signed_qrs(dir: impl AsRef<Path>, config: &AppConfig) -> Result<()> {
    let all_qrs = qrs_in_dir(&dir)?;
    let mut chain_verifiers_map = HashMap::new();
    for chain in &config.chains {
        chain_verifiers_map.insert(
            chain.name.clone().to_lowercase(),
            config.verifiers.get(&chain.verifier).unwrap(),
        );
    }

    // Quick check that latest files are signed
    for qr in &all_qrs {
        ensure!(qr.file_name.is_signed, "{} is not signed", qr.file_name);
    }

    for qr in &all_qrs {
        if let ContentType::Metadata(_) = qr.file_name.content_type {
            let f_name = &qr.file_name;
            match validate_metadata_qr(qr, &chain_verifiers_map) {
                Ok(_) => info!("🎉 {} is verified!", f_name),
                Err(e) => bail!("failed to verify {}: {}", f_name, e),
            }
        }
    }
    Ok(())
}

fn validate_metadata_qr(
    qr_path: &QrPath,
    chain_verifier_map: &HashMap<String, &Verifier>,
) -> Result<()> {
    ensure!(
        qr_path.file_name.is_signed,
        "{} is not signed",
        qr_path.file_name
    );

    let data_hex = read_qr_file(&qr_path.to_path_buf())?;
    let signed =
        pass_crypto(&data_hex, TransferContent::LoadMeta).map_err(|e| anyhow!("{:?}", e))?;
    let encryption = match &signed.verifier.v {
        Some(VerifierValue::Standard { m }) => multisigner_to_encryption(m),
        _ => bail!(
            "unable to get verifier key from qr file: {:?}",
            &signed.verifier
        ),
    };

    let (meta, _) = ContentLoadMeta::from_slice(&signed.message)
        .meta_genhash()
        .map_err(|e| anyhow!("{:?}", e))?;
    let meta_values = MetaValues::from_slice_metadata(&meta).map_err(|e| anyhow!("{:?}", e))?;

    let verifier = &chain_verifier_map
        .get(&meta_values.name.to_lowercase())
        .unwrap();
    let public_key = match encryption {
        Encryption::Sr25519 => &verifier.public_key,
        Encryption::Ethereum | Encryption::Ecdsa => verifier.ethereum_public_key.as_ref().unwrap(),
        _ => bail!("unsupported verifier type: {:?}", &signed.verifier),
    };

    verify_signature(&signed.verifier, public_key)?;

    verify_filename(&meta_values, &qr_path.file_name)?;
    Ok(())
}

fn verify_signature(verifier: &NetworkVerifier, public_key: &str) -> Result<()> {
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
        actual_qr_name == &expected,
        "filename mismatch! Expected {}, got {}",
        expected,
        actual_qr_name
    );
    Ok(())
}
