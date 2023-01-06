mod generate;
// mod github;
// mod wasm;

use std::env;
use std::str::FromStr;

use blake2_rfc::blake2b::blake2b;
use generate::{generate_signed_metadata_qr, generate_signed_spec_qr};
use log::{info, warn};
use sp_core::H256;
use sp_core::{sr25519, Pair};

use crate::config::AppConfig;
use crate::fetch::Fetcher;
use crate::qrs::{find_metadata_qrs, find_spec_qrs};
use crate::source::{save_source_info, Source};
use crate::updater::github::fetch_latest_runtime;
use crate::updater::wasm::{download_wasm, meta_values_from_wasm_bytes};

pub(crate) fn autosign_from_node(config: AppConfig, fetcher: impl Fetcher) -> anyhow::Result<()> {
    log::debug!("autosign_from_node()");

    // let devsecret = "caution juice atom organ advance problem want pledge someone senior holiday very";

    let secret_key = "SIGNING_SEED_PHRASE";
    let secret_seed_phrase = match env::var(secret_key) {
        Ok(value) => value,
        Err(e) => {
            log::error!("Could not interpret environment variable: {secret_key}: {e}");
            panic!();
        }
    };

    let sr25519_pair = match sr25519::Pair::from_string(&secret_seed_phrase, None) {
        Ok(pair) => pair,
        Err(e) => {
            log::error!("Error: Bad secret seed phrase {e:?}");
            panic!();
        }
    };

    let metadata_qrs = find_metadata_qrs(&config.qr_dir)?;
    let specs_qrs = find_spec_qrs(&config.qr_dir)?;

    let mut is_changed = false;
    for chain in config.chains {
        log::debug!("chain={}", chain.name.as_str());

        // CHAIN/NETWORK SPECS

        // Check to see if the chain specs already exist
        if !specs_qrs.contains_key(chain.name.as_str()) {
            let network_specs = fetcher.fetch_specs(&chain)?;
            generate_signed_spec_qr(&sr25519_pair, &network_specs, &config.qr_dir)?;
            is_changed = true;
        }

        // METADATA
        let fetched_meta = fetcher.fetch_metadata(&chain)?;
        let version = fetched_meta.meta_values.version;

        // Check to see if the metadata version has changed
        if let Some(map) = metadata_qrs.get(&chain.name) {
            if map.contains_key(&version) {
                continue;
            }
        }

        let path = generate_signed_metadata_qr(
            &sr25519_pair,
            &fetched_meta.meta_values,
            &fetched_meta.genesis_hash,
            &config.qr_dir,
        )?;

        info!("💾 Saving source metadata.");
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
pub(crate) async fn autosign_from_github(config: AppConfig) -> anyhow::Result<()> {
    log::debug!("autosign_from_github()");

    // let devsecret = "caution juice atom organ advance problem want pledge someone senior holiday very";

    let secret_key = "SIGNING_SEED_PHRASE";
    let secret_seed_phrase = match env::var(secret_key) {
        Ok(value) => value,
        Err(e) => {
            log::error!("Could not interpret environment variable: {secret_key}: {e}");
            panic!();
        }
    };

    let sr25519_pair = match sr25519::Pair::from_string(&secret_seed_phrase, None) {
        Ok(pair) => pair,
        Err(e) => {
            log::error!("Error: Bad secret seed phrase {e:?}");
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
            warn!("🤨 No releases found for {}", chain.name);
            continue;
        }
        let wasm = wasm.unwrap();
        info!("📅 Found VERSION version {}", wasm.version);

        // Check to see if there is newer metadata
        if let Some(map) = metadata_qrs.get(&chain.name) {
            let mut needs_updating: bool = false;
            let num_keys: usize = map.keys().len();
            info!("Checking {} files.", num_keys);
            for metadata_file_version in map.keys() {
                info!("Comparing {} > {} for {}",  &wasm.version, metadata_file_version, &chain.name);
                if &wasm.version > metadata_file_version {
                    needs_updating = true;
                    info!(
                        "The GitHub version {} for {} is newer than {} in the file.",
                        &wasm.version, &chain.name, metadata_file_version
                    );
                    break;
                }
            }
            if !needs_updating {
                info!("🎉 {} is up to date!", chain.name);
                continue;
            }
        }

        // Generate a new signed QR bar code
        let wasm_bytes = download_wasm(wasm.to_owned()).await?;
        let meta_hash = blake2b(32, &[], &wasm_bytes).as_bytes().to_vec();
        let meta_values = meta_values_from_wasm_bytes(&wasm_bytes)?;
        let genesis_hash = H256::from_str(&github_repo.genesis_hash).unwrap();
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
