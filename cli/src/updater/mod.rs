mod generate;
mod github;
pub(crate) mod source;
mod wasm;

use std::str::FromStr;

use blake2_rfc::blake2b::blake2b;
use log::{info, warn};
use sp_core::H256;

use crate::config::AppConfig;
use crate::fetch::Fetcher;
use crate::qrs::{metadata_files, spec_files};
use crate::source::{save_source_info, Source};
use crate::updater::generate::{generate_metadata_qr, generate_spec_qr};
use crate::updater::github::fetch_latest_runtime;
use crate::updater::wasm::{download_wasm, meta_values_from_wasm_bytes};

pub(crate) fn update_from_node(config: AppConfig, fetcher: impl Fetcher) -> anyhow::Result<()> {
    let metadata_qrs = metadata_files(&config.qr_dir)?;
    let specs_qrs = spec_files(&config.qr_dir)?;

    let mut is_changed = false;
    for chain in config.chains {
        if !specs_qrs.contains_key(&chain.portal_id()) {
            let specs = fetcher.fetch_specs(&chain)?;
            generate_spec_qr(&specs, &config.qr_dir, &chain.portal_id())?;
            is_changed = true;
        }

        info!("🔍 Checking for updates for {}", chain.name);
        let fetched_meta = match fetcher.fetch_metadata(&chain) {
            Ok(meta) => meta,
            Err(e) => {
                warn!("🤨 Failed to fetch metadata: {:?}", e);
                continue;
            }
        };
        let version = fetched_meta.meta_values.version;

        // Skip if already have QR for the same version
        if let Some(map) = metadata_qrs.get(&chain.portal_id()) {
            if map.contains_key(&version) {
                continue;
            }
        }
        let path = generate_metadata_qr(
            &fetched_meta.meta_values,
            &fetched_meta.genesis_hash,
            &config.qr_dir,
            &chain.portal_id(),
        )?;
        let source = Source::Rpc {
            block: fetched_meta.block_hash,
        };
        save_source_info(&path, &source)?;
        is_changed = true;
    }

    if !is_changed {
        info!("🎉 Everything is up to date!");
    }
    Ok(())
}

#[tokio::main]
pub(crate) async fn update_from_github(config: AppConfig) -> anyhow::Result<()> {
    let metadata_qrs = metadata_files(&config.qr_dir)?;
    for chain in config.chains {
        info!("🔍 Checking for updates for {}", chain.name);
        if chain.github_release.is_none() {
            info!("↪️ No GitHub releases configured, skipping",);
            continue;
        }

        let github_repo = chain.github_release.as_ref().unwrap();
        let wasm = fetch_latest_runtime(github_repo, &chain.name).await?;
        if wasm.is_none() {
            warn!("🤨 No releases found");
            continue;
        }
        let wasm = wasm.unwrap();
        info!("📅 Found version {}", wasm.version);
        let genesis_hash = H256::from_str(&github_repo.genesis_hash).unwrap();

        // Skip if already have QR for the same version
        if let Some(map) = metadata_qrs.get(&chain.portal_id()) {
            if map.contains_key(&wasm.version) || map.keys().min().unwrap_or(&0) > &wasm.version {
                info!("🎉 {} is up to date!", chain.name);
                continue;
            }
        }
        let wasm_bytes = download_wasm(wasm.to_owned()).await?;
        let meta_hash = blake2b(32, &[], &wasm_bytes).as_bytes().to_vec();
        let meta_values = meta_values_from_wasm_bytes(&wasm_bytes)?;
        let path = generate_metadata_qr(
            &meta_values,
            &genesis_hash,
            &config.qr_dir,
            &chain.portal_id(),
        )?;
        let source = Source::Wasm {
            github_repo: format!("{}/{}", github_repo.owner, github_repo.repo),
            hash: format!("0x{}", hex::encode(meta_hash)),
        };
        save_source_info(&path, &source)?;
    }
    Ok(())
}
