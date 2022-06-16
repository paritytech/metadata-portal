mod generate;
mod github;
pub(crate) mod source;
mod wasm;

use crate::config::AppConfig;
use crate::lib::string::hex_to_bytes;
use anyhow::Context;

use crate::fetch::Fetcher;
use crate::qrs::{find_metadata_qrs, find_spec_qrs};
use log::info;

use crate::updater::generate::{generate_metadata_qr, generate_spec_qr};
use crate::updater::github::fetch_release_runtimes;
use crate::updater::wasm::meta_values_from_wasm;

pub(crate) fn update_from_node(config: AppConfig, fetcher: impl Fetcher) -> anyhow::Result<()> {
    let metadata_qrs = find_metadata_qrs(&config.qr_dir)?;
    let specs_qrs = find_spec_qrs(&config.qr_dir)?;

    let mut is_changed = false;
    for chain in config.chains {
        let meta_specs = fetcher.fetch_chain_info(&chain)?;
        let version = meta_specs.meta_values.version;

        if !specs_qrs.contains_key(chain.name.as_str()) {
            generate_spec_qr(&meta_specs, &config.qr_dir)?;
            is_changed = true;
        }

        // Skip if already have QR for the same version
        if let Some(map) = metadata_qrs.get(&chain.name) {
            if map.contains_key(&version) {
                continue;
            }
        }
        generate_metadata_qr(
            &meta_specs.meta_values,
            meta_specs.specs.genesis_hash,
            &config.qr_dir,
        )?;
        is_changed = true;
    }

    if !is_changed {
        info!("🎉 Everything is up to date!");
    }
    Ok(())
}

#[tokio::main]
pub(crate) async fn update_from_github(config: AppConfig) -> anyhow::Result<()> {
    if config.github.is_none() {
        info!("↪️ No GitHub repository specified, skipping update");
        return Ok(());
    }
    let runtimes = fetch_release_runtimes(&config.github.unwrap()).await?;
    info!("📦 Found {} runtimes", runtimes.len());
    let metadata_qrs = find_metadata_qrs(&config.qr_dir)?;
    let mut left_to_update = config.chains.len();
    for chain in config.chains {
        if !runtimes.contains_key(&chain.name) {
            info!("🤨 No releases for {} found", chain.name);
            continue;
        }
        let wasm = runtimes.get(&chain.name).unwrap();
        let genesis_hash = chain.genesis_hash.context(format!(
            "cannot find genesis_hash for {} in config.toml",
            chain.name
        ))?;
        let genesis_hash = hex_to_bytes(&genesis_hash)?.try_into().unwrap();

        // Skip if already have QR for the same version
        if let Some(map) = metadata_qrs.get(&chain.name) {
            if map.contains_key(&wasm.version) {
                left_to_update -= 1;
                continue;
            }
        }
        let meta_values = meta_values_from_wasm(wasm.to_owned()).await?;
        generate_metadata_qr(&meta_values, genesis_hash, &config.qr_dir)?;
    }
    if left_to_update == 0 {
        info!("🎉 Everything is up to date!");
    }
    Ok(())
}
