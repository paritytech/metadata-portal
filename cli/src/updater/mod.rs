mod generate;
mod github;
pub(crate) mod source;
mod wasm;

use std::process::exit;
use std::str::FromStr;

use blake2_rfc::blake2b::blake2b;
use log::{info, warn};
use sp_core::H256;

use crate::config::AppConfig;
use crate::fetch::Fetcher;
use crate::qrs::{find_metadata_qrs, find_spec_qrs};
use crate::source::{save_source_info, Source};
use crate::updater::generate::{download_metadata_qr, generate_metadata_qr, generate_spec_qr};
use crate::updater::github::fetch_latest_runtime;
use crate::updater::wasm::{download_wasm, meta_values_from_wasm_bytes};
use crate::utils::types::get_crypto;

pub(crate) fn update_from_node(
    config: AppConfig,
    sign: bool,
    signing_key: String,
    fetcher: impl Fetcher,
) -> anyhow::Result<()> {
    let metadata_qrs = find_metadata_qrs(&config.qr_dir)?;
    let specs_qrs = find_spec_qrs(&config.qr_dir)?;
    let mut is_changed = false;
    let mut error_fetching_data = false;
    for chain in config.chains {
        let encryption = get_crypto(&chain);
        if !specs_qrs.contains_key(chain.name.as_str()) {
            let specs_res = fetcher.fetch_specs(&chain);
            if specs_res.is_err() {
                error_fetching_data = true;
                warn!(
                    "Can't get specs for {}. Error is {}",
                    chain.name,
                    specs_res.err().unwrap()
                );
                continue;
            }
            if chain.verifier == "parity" {
                warn!("The chain {} should be added and signed by Parity, please check it on the Parity Metadata portal https://metadata.parity.io/", chain.name);
            } else {
                generate_spec_qr(
                    &specs_res.unwrap(),
                    &config.qr_dir,
                    sign,
                    signing_key.to_owned(),
                    &encryption,
                )?;
            }
            is_changed = true;
        }

        let fetched_meta_res = fetcher.fetch_metadata(&chain);
        if fetched_meta_res.is_err() {
            error_fetching_data = true;
            warn!(
                "Can't get metadata for {}. Error is {}",
                chain.name,
                fetched_meta_res.err().unwrap()
            );
            continue;
        }
        let fetched_meta = fetched_meta_res.unwrap();
        let version = fetched_meta.meta_values.version;

        // Skip if already have QR for the same version
        if let Some(map) = metadata_qrs.get(&chain.name) {
            if map.contains_key(&version) {
                continue;
            }
        }
        if chain.verifier == "parity" {
            download_metadata_qr(
                "https://metadata.parity.io/qr",
                &fetched_meta.meta_values,
                &config.qr_dir,
            )?;
        } else {
            let path = generate_metadata_qr(
                &fetched_meta.meta_values,
                &fetched_meta.genesis_hash,
                &config.qr_dir,
                sign,
                signing_key.to_owned(),
                &encryption,
            )?;
            let source = Source::Rpc {
                block: fetched_meta.block_hash,
            };
            save_source_info(&path, &source)?;
        }
        is_changed = true;
    }

    if error_fetching_data {
        warn!("⚠️ Some chain data wasn't read. Please check the log!");
        exit(12);
    }

    if !is_changed {
        info!("🎉 Everything is up to date!");
    }

    Ok(())
}

#[tokio::main]
pub(crate) async fn update_from_github(
    config: AppConfig,
    sign: bool,
    signing_key: String,
) -> anyhow::Result<()> {
    let metadata_qrs = find_metadata_qrs(&config.qr_dir)?;
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
        if let Some(map) = metadata_qrs.get(&chain.name) {
            if map.contains_key(&wasm.version) || map.keys().min().unwrap_or(&0) > &wasm.version {
                info!("🎉 {} is up to date!", chain.name);
                continue;
            }
        }
        let wasm_bytes = download_wasm(wasm.to_owned()).await?;
        let meta_hash = blake2b(32, &[], &wasm_bytes).as_bytes().to_vec();
        let meta_values = meta_values_from_wasm_bytes(&wasm_bytes)?;
        let encryption = get_crypto(&chain);
        let path = generate_metadata_qr(
            &meta_values,
            &genesis_hash,
            &config.qr_dir,
            sign,
            signing_key.to_owned(),
            &encryption,
        )?;
        let source = Source::Wasm {
            github_repo: format!("{}/{}", github_repo.owner, github_repo.repo),
            hash: format!("0x{}", hex::encode(meta_hash)),
        };
        save_source_info(&path, &source)?;
    }
    Ok(())
}
