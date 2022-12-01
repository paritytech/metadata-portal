mod generate;
// mod github;
// mod wasm;

use std::str::FromStr;

use blake2_rfc::blake2b::blake2b;
use generate::{generate_signed_metadata_qr, generate_signed_spec_qr};
use log::{info, warn};
use sp_core::H256;
use sp_core::{sr25519, Pair};

use crate::config::AppConfig;
use crate::fetch::Fetcher;
use crate::qrs::{find_metadata_qrs, find_spec_qrs, next_metadata_version};
use crate::source::{save_source_info, Source};
use crate::updater::github::fetch_latest_runtime;
use crate::updater::wasm::{download_wasm, meta_values_from_wasm_bytes};
use crate::collector::export::export_specs;
use crate::collector::file::save_to_file;
use crate::fetch::RpcFetcher;

pub(crate) fn autosign_from_node(config: AppConfig, fetcher: impl Fetcher) -> anyhow::Result<()> {
    log::debug!("autosign_from_node()");

    let specs = export_specs(&config, RpcFetcher)?;
    save_to_file(&specs, config.data_file)?;

    let secret = "caution juice atom organ advance problem want pledge someone senior holiday very";
    let sr25519_pair = match sr25519::Pair::from_string(secret, None) {
        Ok(pair) => pair,
        Err(e) => {
            log::error!("Error: Bad secret seed phrase {e:?}");
            panic!();
        }
    };

    let mut is_changed = false;
    for chain in config.chains {
        let network_specs = fetcher.fetch_specs(&chain)?;
        // (&specs, &config.qr_dir)?;
        is_changed = true;

        log::debug!("chain={}", chain.name.as_str());

        let path = generate_signed_spec_qr(&sr25519_pair, &network_specs, &config.qr_dir)?;

        // println!("sr25519_pair={}", sr25519_pair);

        let fetched_meta = fetcher.fetch_metadata(&chain)?;
        let version = fetched_meta.meta_values.version;

        // // Skip if already have QR for the same version
        // if let Some(map) = metadata_qrs.get(&chain.name) {
        //     if map.contains_key(&version) {
        //         continue;
        //     }
        // }

        let path = generate_signed_metadata_qr(
            &sr25519_pair,
            &fetched_meta.meta_values,
            &fetched_meta.genesis_hash,
            &config.qr_dir,
        )?;

        info!("Saving source metadata.");
        let source = Source::Rpc {
            block: fetched_meta.block_hash,
        };
        save_source_info(&path, &source)?;

        // is_changed = true;
    }

    if !is_changed {
        info!("🎉 Everything is up to date!");
    }
    Ok(())
}

#[tokio::main]
pub(crate) async fn autosign_from_github(config: AppConfig) -> anyhow::Result<()> {
    log::debug!("autosign_from_github()");

    let specs = export_specs(&config, RpcFetcher)?;
    save_to_file(&specs, config.data_file)?;

    let secret = "caution juice atom organ advance problem want pledge someone senior holiday very";
    let sr25519_pair = match sr25519::Pair::from_string(secret, None) {
        Ok(pair) => pair,
        Err(e) => {
            log::error!("Error: Bad secret seed phrase. {e:?}");
            panic!();
        }
    };

    let metadata_qrs = find_metadata_qrs(&config.qr_dir)?;
    for chain in config.chains {
        info!("🔍 Checking for updates for {}", chain.name);
        if chain.github_release.is_none() {
            info!("↪️ No GitHub releases configured, skipping",);
            continue;
        }

        let github_repo = chain.github_release.unwrap();
        let wasm = fetch_latest_runtime(&github_repo, &chain.name).await?;
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
        let path = generate_signed_metadata_qr(
            &sr25519_pair,
            &meta_values,
            &genesis_hash,
            &config.qr_dir,
        )?;
        info!("Saving source metadata.");
        let source = Source::Wasm {
            github_repo: format!("{}/{}", github_repo.owner, github_repo.repo),
            hash: format!("0x{}", hex::encode(meta_hash)),
        };
        save_source_info(&path, &source)?;
    }
    Ok(())
}
