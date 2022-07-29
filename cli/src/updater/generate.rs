use std::path::{Path, PathBuf};

use definitions::metadata::MetaValues;
use definitions::network_specs::NetworkSpecsToSend;
use definitions::qr_transfers::{ContentAddSpecs, ContentLoadMeta};
use generate_message::full_run;
use generate_message::parser::{Command as SignerCommand, Crypto, Goal, Make, Msg};
use log::info;
use sp_core::H256;

use crate::lib::path::{ContentType, QrFileName};

pub(crate) fn generate_metadata_qr(
    meta_values: &MetaValues,
    genesis_hash: &H256,
    target_dir: &Path,
) -> anyhow::Result<PathBuf> {
    let content = ContentLoadMeta::generate(&meta_values.meta, genesis_hash);

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
    full_run(SignerCommand::Make(make)).map_err(anyhow::Error::msg)?;
    Ok(path)
}

pub(crate) fn generate_spec_qr(
    specs: &NetworkSpecsToSend,
    target_dir: &Path,
) -> anyhow::Result<PathBuf> {
    let file_name =
        QrFileName::new(&specs.name.to_lowercase(), ContentType::Specs, false).to_string();
    let path = target_dir.join(&file_name);
    let content = ContentAddSpecs::generate(specs);

    let make = Make {
        goal: Goal::Qr,
        crypto: Crypto::None,
        msg: Msg::AddSpecs(content.to_sign()),
        name: Some(path.to_str().unwrap().to_owned()),
    };
    info!("⚙️  Generating {}...", file_name);
    full_run(SignerCommand::Make(make)).map_err(anyhow::Error::msg)?;
    Ok(path)
}
