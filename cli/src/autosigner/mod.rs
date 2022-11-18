use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use definitions::metadata::MetaValues;
use definitions::network_specs::NetworkSpecsToSend;
use definitions::qr_transfers::{ContentAddSpecs, ContentLoadMeta};
use generate_message::full_run;
use generate_message::parser::{
    Command as SignerCommand, Goal, Make, Msg, Signature, Sufficient, Verifier,
};
use log::info;
use sp_core::H256;

use crate::lib::path::{ContentType, QrFileName};

pub(crate) fn autosign(config: AppConfig) -> anyhow::Result<()> {
    log::debug!("autosign()");

    let key = "SIGNING_SEED_PHRASE";
    match env::var(key) {
        Ok(val) => println!("{key}: {val:?}"),
        Err(e) => println!("couldn't interpret {key}: {e}"),
    }

    Ok(())
}

pub(crate) fn generate_signed_metadata_qr(
    meta_values: &MetaValues,
    genesis_hash: &H256,
    target_dir: &Path,
) -> anyhow::Result<PathBuf> {
    log::debug!("generate_signed_metadata_qr()");

    // Create a filename for the signed metadata.
    // e.g. {chain_name}_metadata_{spec_version}.apng
    let file_name = QrFileName::new(
        &meta_values.name.to_lowercase(),
        ContentType::Metadata(meta_values.version),
        true,
    )
    .to_string();
    let path = target_dir.join(&file_name);

    // The content for the bar code
    let content = ContentLoadMeta::generate(&meta_values.meta, genesis_hash);

    // Generate a signed QR barcode for the metadata using the filename
    info!("⚙️  Generating {}...", file_name);
    generate_signed_qr(&content.to_sign(), &path, Msg::LoadMetadata, signature)?;
    Ok(path)
}

pub(crate) fn generate_signed_spec_qr(
    specs: &NetworkSpecsToSend,
    target_dir: &Path,
) -> anyhow::Result<PathBuf> {
    log::debug!("generate_signed_spec_qr()");

    // Create a filename for the signed specs.
    // e.g. {chain_name}_specs.png
    let file_name =
        QrFileName::new(&specs.name.to_lowercase(), ContentType::Specs, true).to_string();
    let path = target_dir.join(&file_name);

    // The content for the bar code
    let content = ContentAddSpecs::generate(specs);

    // Generate a signed QR barcode for specs using the filename
    info!("⚙️  Generating {}...", file_name);
    generate_signed_qr(&content.to_sign(), &path, Msg::AddSpecs, signature)?;
    Ok(path)
}

fn generate_signed_qr(
    content: &[u8],
    signed_qr: &QrPath,
    msg_type: Msg,
    signature: String,
) -> anyhow::Result<QrPath> {
    log::debug!("generate_signed_qr()");

    let transfer_content = match signed_qr.file_name.content_type {
        ContentType::Metadata(_) => TransferContent::LoadMeta,
        ContentType::Specs => TransferContent::AddSpecs,
    };
    let passed_crypto = pass_crypto(&content, transfer_content).map_err(|e| anyhow!("{:?}", e))?;

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
        payload: content_file,
        export_dir: signed_qr.dir.clone(),
        crypto: Some(Encryption::Sr25519),
    };
    println!("⚙ generating {}...", signed_qr);
    full_run(SignerCommand::Make(make)).map_err(|e| anyhow!("{:?}", e))?;
    // Preserve png source information
    if let Some(png_source) = read_png_source(&unsigned_qr.to_path_buf())? {
        save_source_info(&signed_qr.to_path_buf(), &png_source)?;
    };
    Ok(signed_qr)
}
