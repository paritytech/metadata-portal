use crate::lib::path::{ContentType, QrFileName};
use crate::updater::export::MetaSpecs;

use definitions::metadata::MetaValues;
use definitions::qr_transfers::{ContentAddSpecs, ContentLoadMeta};
use generate_message::make_message::make_message;
use generate_message::parser::{Crypto, Goal, Make, Msg};
use log::info;
use std::path::{Path, PathBuf};

pub(crate) fn generate_metadata_qr(
    meta_values: &MetaValues,
    genesis_hash: [u8; 32],
    target_dir: &Path,
) -> anyhow::Result<PathBuf> {
    let content = ContentLoadMeta::generate(&meta_values.meta, &genesis_hash);

    let file_name = QrFileName::new(
        &meta_values.name.to_lowercase(),
        ContentType::Metadata(meta_values.version),
        false,
    )
    .to_string();
    let path = target_dir.join(&file_name);

    let make = Make {
        goal: Goal::Qr,
        crypto: Crypto::None,
        msg: Msg::LoadMetadata(content.to_sign()),
        name: Some(path.to_str().unwrap().to_owned()),
    };
    info!("⚙️  Generating {}...", file_name);
    make_message(make).map_err(anyhow::Error::msg)?;
    Ok(path)
}

pub(crate) fn generate_spec_qr(
    meta_specs: &MetaSpecs,
    target_dir: &Path,
) -> anyhow::Result<PathBuf> {
    let file_name = QrFileName::new(
        &meta_specs.meta_values.name.to_lowercase(),
        ContentType::Specs,
        false,
    )
    .to_string();
    let path = target_dir.join(&file_name);
    let content = ContentAddSpecs::generate(&meta_specs.specs);

    let make = Make {
        goal: Goal::Qr,
        crypto: Crypto::None,
        msg: Msg::AddSpecs(content.to_sign()),
        name: Some(path.to_str().unwrap().to_owned()),
    };
    info!("⚙️  Generating {}...", file_name);
    make_message(make).map_err(anyhow::Error::msg)?;
    Ok(path)
}
