use std::convert::TryInto;
use std::path::PathBuf;
use definitions::qr_transfers::ContentLoadMeta;
use generate_message::make_message::make_message;
use generate_message::parser::{Crypto, Goal, Make, Msg};
use qr_lib::path::QrFileName;
use qr_lib::read::hex_to_bytes;
use crate::export::MetaSpecs;


pub fn generate_metadata_qr(meta_specs: &MetaSpecs, target_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    let genesis_hash: [u8; 32] = hex_to_bytes(&meta_specs.specs.genesis_hash)?.try_into().unwrap();
    let content = ContentLoadMeta::generate(&meta_specs.meta_values.meta, &genesis_hash);

    let file_name = QrFileName::new(
        &meta_specs.meta_values.name.to_lowercase(),
        meta_specs.meta_values.version,
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
