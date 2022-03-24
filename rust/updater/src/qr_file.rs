use std::path::PathBuf;
use definitions::qr_transfers::{ContentAddSpecs, ContentLoadMeta};
use generate_message::make_message::make_message;
use generate_message::parser::{Crypto, Goal, Make, Msg};
use qr_lib::path::{ContentType, QrFileName};
use crate::export::MetaSpecs;


pub (crate) fn generate_metadata_qr(meta_specs: &MetaSpecs, target_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    let content = ContentLoadMeta::generate(
        &meta_specs.meta_values.meta,
        &meta_specs.specs.genesis_hash
    );

    let file_name = QrFileName::new(
        &meta_specs.meta_values.name.to_lowercase(),
        ContentType::Metadata(meta_specs.meta_values.version),
        false
    ).to_string();
    let path = target_dir.join(&file_name);

    let make = Make {
        goal: Goal::Qr,
        crypto: Crypto::None,
        msg: Msg::LoadMetadata(content.to_sign()),
        name: Some(path.to_str().unwrap().to_owned()),
    };
    println!("⚙ generating {}. It takes a while...", file_name);
    make_message(make).map_err(anyhow::Error::msg)?;
    Ok(path)
}

pub (crate) fn generate_spec_qr(meta_specs: &MetaSpecs, target_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    let file_name = QrFileName::new(
        &meta_specs.meta_values.name.to_lowercase(),
        ContentType::Specs,
        false
    ).to_string();
    let path = target_dir.join(&file_name);
    let content = ContentAddSpecs::generate(&meta_specs.specs);

    let make = Make {
        goal: Goal::Qr,
        crypto: Crypto::None,
        msg: Msg::AddSpecs(content.to_sign()),
        name: Some(path.to_str().unwrap().to_owned()),
    };
    println!("⚙ generating {}...", file_name);
    make_message(make).map_err(anyhow::Error::msg)?;
    Ok(path)
}
