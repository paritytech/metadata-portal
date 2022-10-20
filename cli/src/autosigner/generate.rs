use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use definitions::crypto::{Encryption, SufficientCrypto};
use definitions::error::TransferContent;
use definitions::metadata::MetaValues;
use definitions::network_specs::NetworkSpecsToSend;
use definitions::qr_transfers::{ContentAddSpecs, ContentLoadMeta};
use generate_message::full_run;
use generate_message::parser::{
    Command as SignerCommand, Goal, Make, Msg, Signature, Sufficient, Verifier,
};
use parity_scale_codec::Encode;
use sp_core::crypto::Pair;
use sp_core::H256;
use transaction_parsing::check_signature::pass_crypto;

use crate::lib::path::{ContentType, QrFileName, QrPath};

pub(crate) fn generate_signed_spec_qr(
    pair: &sp_core::sr25519::Pair,
    network_specs: &NetworkSpecsToSend,
    target_dir: &Path,
) -> anyhow::Result<PathBuf> {
    log::debug!("generate_signed_spec_qr()");

    // Create the file name for the QR bar code
    let file_name =
        QrFileName::new(&network_specs.name.to_lowercase(), ContentType::Specs, true).to_string();
    let path = target_dir.join(&file_name);

    let signed_qr = QrPath::try_from(&path).ok().unwrap();

    // Generate the unsigned content
    let content_specs = ContentAddSpecs::generate(network_specs);

    let message_to_verify = content_specs.to_sign();
    let message_to_transfer = content_specs.to_transfer();
    let crypto_type_code = "01";
    let msg_type_code = "c1";
    let msg_type = Msg::AddSpecs;
    let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
    let sig = pair.sign(&message_to_verify[..]);
    let signature = sig.0.to_vec();
    let s = SufficientCrypto::Sr25519 {
        public: pair.clone().public(),
        signature: sig,
    };
    let signature_encoded = SufficientCrypto::encode(&s);
    let signature_hex = hex::encode(&signature_encoded);
    let complete_message = [
        hex::decode(prelude).expect("known value"),
        pair.public().to_vec(),
        message_to_transfer,
        signature,
    ]
    .concat();

    let data_hex = hex::encode(&complete_message);
    let passed_crypto =
        pass_crypto(&data_hex, TransferContent::AddSpecs).map_err(|e| anyhow!("{:?}", e))?;

    let tmp_dir = tempfile::tempdir()?;
    let content_file = tmp_dir.path().join("content");
    let mut f = File::create(&content_file)?;
    f.write_all(passed_crypto.message.deref())?;

    log::debug!("FILE={}", signed_qr);
    log::debug!("SIGNATURE={}", signature_hex);

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
            sufficient_hex: Some(signature_hex),
            sufficient_file: None,
        },
        msg: msg_type,
        name: Some(signed_qr.to_path_buf()),
        files_dir: signed_qr.dir.clone(),
        payload: content_file,
        export_dir: signed_qr.dir.clone(),
        crypto: Some(Encryption::Sr25519),
    };
    println!("⚙ Generating {}...", signed_qr);
    full_run(SignerCommand::Make(make)).map_err(|e| anyhow!("{:?}", e))?;

    Ok(path)
}

pub(crate) fn generate_signed_metadata_qr(
    pair: &sp_core::sr25519::Pair,
    meta_values: &MetaValues,
    genesis_hash: &H256,
    target_dir: &Path,
) -> anyhow::Result<PathBuf> {
    log::debug!("generate_signed_metadata_qr()");

    let content = ContentLoadMeta::generate(&meta_values.meta, genesis_hash);

    let file_name = QrFileName::new(
        &meta_values.name.to_lowercase(),
        ContentType::Metadata(meta_values.version),
        true,
    )
    .to_string();
    let path = target_dir.join(&file_name);
    let signed_qr = QrPath::try_from(&path).ok().unwrap();

    let message_to_verify = content.to_sign();
    let message_to_transfer = content.to_transfer();
    let crypto_type_code = "01";
    let msg_type_code = "80";
    let msg_type = Msg::LoadMetadata;
    let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
    let sig = pair.sign(&message_to_verify[..]);
    let signature = sig.0.to_vec();
    let s = SufficientCrypto::Sr25519 {
        public: pair.clone().public(),
        signature: sig,
    };
    let signature_encoded = SufficientCrypto::encode(&s);
    let signature_hex = hex::encode(&signature_encoded);
    let complete_message = [
        hex::decode(prelude).expect("known value"),
        pair.clone().public().to_vec(),
        message_to_transfer,
        signature,
    ]
    .concat();

    let data_hex = hex::encode(&complete_message);
    let passed_crypto =
        pass_crypto(&data_hex, TransferContent::LoadMeta).map_err(|e| anyhow!("{:?}", e))?;

    let tmp_dir = tempfile::tempdir()?;
    let content_file = tmp_dir.path().join("content");
    let mut f = File::create(&content_file)?;
    f.write_all(passed_crypto.message.deref())?;

    log::debug!("FILE={}", signed_qr);
    log::debug!("SIGNATURE={}", signature_hex);

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
            sufficient_hex: Some(signature_hex),
            sufficient_file: None,
        },
        msg: msg_type,
        name: Some(signed_qr.to_path_buf()),
        files_dir: signed_qr.dir.clone(),
        payload: content_file,
        export_dir: signed_qr.dir.clone(),
        crypto: Some(Encryption::Sr25519),
    };
    println!("⚙ Generating {}...", signed_qr);
    full_run(SignerCommand::Make(make)).map_err(|e| anyhow!("{:?}", e))?;

    Ok(path)
}
