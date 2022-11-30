use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::os::raw;
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
use hex::ToHex;
use log::info;
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

    let qrPath = QrPath::try_from(&path).ok().unwrap();

    // Generate the unsigned content
    let content_specs = ContentAddSpecs::generate(network_specs);

    let message_to_verify = content_specs.to_sign();
    let message_to_transfer = content_specs.to_transfer();
    let crypto_type_code = "01";
    let msg_type_code = "c1";
    let msg_type = Msg::AddSpecs;
    let prelude = format!("53{}{}", crypto_type_code, msg_type_code);
    let sr25519_pair = pair;
    let sig = sr25519_pair.sign(&message_to_verify[..]);
    let signature = sig.0.to_vec();
    let s = SufficientCrypto::Sr25519 {
        public: pair.public(),
        signature: sig,
    };
    let signature_encoded = SufficientCrypto::encode(&s);
    let signature_hex = hex::encode(&signature_encoded);
    let complete_message = [
        hex::decode(prelude).expect("known value"),
        sr25519_pair.public().to_vec(),
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
    f.write_all(&passed_crypto.message.deref())?;

    let signed_qr = qrPath;

    println!("FILE={}", signed_qr);
    println!("SIGNATURE={}", signature_hex);
    println!("MESSAGE={}", hex::encode(&complete_message));

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
        payload: content_file.clone(),
        export_dir: signed_qr.dir.clone(),
        crypto: Some(Encryption::Sr25519),
    };
    println!("⚙ Generating {}...", signed_qr);
    full_run(SignerCommand::Make(make)).map_err(|e| anyhow!("{:?}", e));

    Ok(path)
}

fn sign_qr(
    unsigned_qr: &QrPath,
    data: &str,
    content_type: ContentType,
    signature: String,
) -> anyhow::Result<QrPath> {
    log::debug!("sign_qr({}, {})", unsigned_qr, signature);

    let mut signed_qr = unsigned_qr.clone();
    signed_qr.file_name.is_signed = true;

    let transfer_content = match content_type {
        ContentType::Metadata(_) => TransferContent::LoadMeta,
        ContentType::Specs => TransferContent::AddSpecs,
    };

    let passed_crypto = pass_crypto(data, transfer_content).map_err(|e| anyhow!("{:?}", e))?;

    let msg_type = match content_type {
        ContentType::Metadata(_) => Msg::LoadMetadata,
        ContentType::Specs => Msg::AddSpecs,
    };
    let tmp_dir = tempfile::tempdir()?;
    let content_file = tmp_dir.path().join("content");
    let mut f = File::create(&content_file)?;
    f.write_all(passed_crypto.message.deref())?;

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
        payload: content_file.clone(),
        export_dir: signed_qr.dir.clone(),
        crypto: Some(Encryption::Sr25519),
    };
    println!("⚙ Generating {}...", signed_qr);
    full_run(SignerCommand::Make(make)).map_err(|e| anyhow!("{:?}", e))?;
    // Preserve png source information
    // if let Some(png_source) = read_png_source(&unsigned_qr.to_path_buf())? {
    //     save_source_info(&signed_qr.to_path_buf(), &png_source)?;
    // };
    Ok(signed_qr)
}

pub(crate) fn generate_signed_metadata_qr(
    pair: sp_core::sr25519::Pair,
    meta_values: &MetaValues,
    genesis_hash: &H256,
    target_dir: &Path,
) -> anyhow::Result<PathBuf> {
    log::debug!("generate_signed_metadata_qr()");

    let content = ContentLoadMeta::generate(&meta_values.meta, genesis_hash);
    let signature = pair.sign(&content.to_sign());
    //println!("signature={:?}", signature);

    let file_name = QrFileName::new(
        &meta_values.name.to_lowercase(),
        ContentType::Metadata(meta_values.version),
        true,
    )
    .to_string();
    let path = target_dir.join(&file_name);

    info!("⚙️  Generating {}...", file_name);
    generate_signed_qr(
        &content.to_sign(),
        signature.encode_hex::<String>(),
        &path,
        Msg::LoadMetadata,
    )?;
    Ok(path)
}

fn generate_signed_qr<P>(
    content: &[u8],
    signature: String,
    target_path: P,
    msg_type: Msg,
) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let tmp_dir = tempfile::tempdir()?;
    let tmp_f_path = tmp_dir.path().join("content");
    let mut content_file = File::create(&tmp_f_path)?;
    content_file.write_all(content)?;

    let files_dir = target_path.as_ref().parent().unwrap().to_path_buf();
    log::debug!(
        "files_dir={}",
        files_dir.clone().into_os_string().into_string().unwrap()
    );

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
        name: Some(target_path.as_ref().to_owned()),
        files_dir: files_dir.clone(),
        payload: tmp_f_path,
        export_dir: files_dir,
        crypto: Some(Encryption::Sr25519),
    };
    full_run(SignerCommand::Make(make)).map_err(|e| anyhow!("{:?}", e))
}

fn generate_unsigned_qr<P>(content: &[u8], target_path: P, msg_type: Msg) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let tmp_dir = tempfile::tempdir()?;
    let tmp_f_path = tmp_dir.path().join("content");
    let mut content_file = File::create(&tmp_f_path)?;
    content_file.write_all(content)?;

    let files_dir = target_path.as_ref().parent().unwrap().to_path_buf();

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
            sufficient_hex: None,
            sufficient_file: None,
        },
        msg: msg_type,
        name: Some(target_path.as_ref().to_owned()),
        files_dir: files_dir.clone(),
        payload: tmp_f_path,
        export_dir: files_dir,
        crypto: None,
    };
    full_run(SignerCommand::Make(make)).map_err(|e| anyhow!("{:?}", e))
}
