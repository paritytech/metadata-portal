use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use definitions::metadata::MetaValues;
use definitions::network_specs::NetworkSpecs;
use definitions::qr_transfers::{ContentAddSpecs, ContentLoadMeta};
use generate_message::full_run;
use generate_message::parser::{
    Command as SignerCommand, Goal, Make, Msg, Signature, Sufficient, Verifier,
};
use log::info;
use sp_core::H256;

use crate::common::path::{ContentType, QrFileName};

pub(crate) fn generate_metadata_qr(
    meta_values: &MetaValues,
    genesis_hash: &H256,
    target_dir: &Path,
    portal_id: &str,
) -> anyhow::Result<PathBuf> {
    let content = ContentLoadMeta::generate(&meta_values.meta, genesis_hash);

    let file_name = QrFileName::new(
        &portal_id.to_lowercase(),
        ContentType::Metadata(meta_values.version),
        false,
    )
    .to_string();
    let path = target_dir.join(&file_name);

    info!("⚙️  Generating {}...", file_name);
    generate_unsigned_qr(&content.to_sign(), &path, Msg::LoadMetadata)?;
    Ok(path)
}

pub(crate) fn generate_spec_qr(
    specs: &NetworkSpecs,
    target_dir: &Path,
    portal_id: &str,
) -> anyhow::Result<PathBuf> {
    let file_name =
        QrFileName::new(&portal_id.to_lowercase(), ContentType::Specs, false).to_string();
    let path = target_dir.join(&file_name);
    let content = ContentAddSpecs::generate(specs);
    info!("⚙️  Generating {}...", file_name);
    generate_unsigned_qr(&content.to_sign(), &path, Msg::AddSpecs)?;
    Ok(path)
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
