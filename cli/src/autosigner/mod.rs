mod generate;
// mod github;
// pub(crate) mod source;
// mod wasm;

use generate::generate_signed_spec_qr;
use log::info;
use sp_core::{sr25519, Pair};

use crate::config::AppConfig;
use crate::fetch::Fetcher;

pub(crate) fn autosign_from_node(config: AppConfig, fetcher: impl Fetcher) -> anyhow::Result<()> {
    log::debug!("autosign_from_node()");

    let secret = "caution juice atom organ advance problem want pledge someone senior holiday very";
    let sr25519_pair = match sr25519::Pair::from_string(secret, None) {
        Ok(pair) => pair,
        Err(e) => {
            log::error!("Bad secret seed phrase");
            panic!();
        }
    };

    let mut is_changed = false;
    for chain in config.chains {
        let network_specs = fetcher.fetch_specs(&chain)?;
        // (&specs, &config.qr_dir)?;
        is_changed = true;

        println!("chain={}", chain.name.as_str());

        generate_signed_spec_qr(&sr25519_pair, &network_specs, &config.qr_dir);

        // println!("sr25519_pair={}", sr25519_pair);

        let fetched_meta = fetcher.fetch_metadata(&chain)?;
        let version = fetched_meta.meta_values.version;

        // // Skip if already have QR for the same version
        // if let Some(map) = metadata_qrs.get(&chain.name) {
        //     if map.contains_key(&version) {
        //         continue;
        //     }
        // }
        // let path = generate_metadata_qr(
        //     &fetched_meta.meta_values,
        //     &fetched_meta.genesis_hash,
        //     &config.qr_dir,
        // )?;
        // let source = Source::Rpc {
        //     block: fetched_meta.block_hash,
        // };
        // save_source_info(&path, &source)?;
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

    // let metadata_qrs = find_metadata_qrs(&config.qr_dir)?;
    // for chain in config.chains {
    //     info!("🔍 Checking for updates for {}", chain.name);
    //     if chain.github_release.is_none() {
    //         info!("↪️ No GitHub releases configured, skipping",);
    //         continue;
    //     }

    //     let github_repo = chain.github_release.unwrap();
    //     let wasm = fetch_latest_runtime(&github_repo, &chain.name).await?;
    //     if wasm.is_none() {
    //         warn!("🤨 No releases found");
    //         continue;
    //     }
    //     let wasm = wasm.unwrap();
    //     info!("📅 Found version {}", wasm.version);
    //     let genesis_hash = H256::from_str(&github_repo.genesis_hash).unwrap();

    //     // Skip if already have QR for the same version
    //     if let Some(map) = metadata_qrs.get(&chain.name) {
    //         if map.contains_key(&wasm.version) || map.keys().min().unwrap_or(&0) > &wasm.version {
    //             info!("🎉 {} is up to date!", chain.name);
    //             continue;
    //         }
    //     }
    //     let wasm_bytes = download_wasm(wasm.to_owned()).await?;
    //     let meta_hash = blake2b(32, &[], &wasm_bytes).as_bytes().to_vec();
    //     let meta_values = meta_values_from_wasm_bytes(&wasm_bytes)?;
    //     let path = generate_metadata_qr(&meta_values, &genesis_hash, &config.qr_dir)?;
    //     let source = Source::Wasm {
    //         github_repo: format!("{}/{}", github_repo.owner, github_repo.repo),
    //         hash: format!("0x{}", hex::encode(meta_hash)),
    //     };
    //     save_source_info(&path, &source)?;
    // }
    Ok(())
}
