mod generate;
// mod github;
// pub(crate) mod source;
// mod wasm;

use std::str::FromStr;

use blake2_rfc::blake2b::blake2b;
use hex::ToHex;
use log::{info, warn};
use sp_core::H256;

use crate::config::AppConfig;
use crate::fetch::Fetcher;
use crate::qrs::{find_metadata_qrs, find_spec_qrs};
use crate::source::{save_source_info, Source};
use crate::updater::github::fetch_latest_runtime;
use crate::updater::wasm::{download_wasm, meta_values_from_wasm_bytes};
use sp_core::{sr25519, Pair};


// pub(crate) fn autosign(config: AppConfig) -> anyhow::Result<()> {
//     log::debug!("autosign()");

//     let key = "SIGNING_SEED_PHRASE";
//     match env::var(key) {
//         Ok(val) => {
//             println!("{key}: {val:?}");
//             //let pair = Pair::from_phrase(&val);
//         },
//         Err(e) => println!("couldn't interpret {key}: {e}"),
//     }

//     // Private key (hex)
//     // 0xc8fa03532fb22ee1f7f6908b9c02b4e72483f0dbd66e4cd456b8f34c6230b849

//     Ok(())
// }

pub(crate) fn autosign_from_node(config: AppConfig, fetcher: impl Fetcher) -> anyhow::Result<()> {
    log::debug!("autosign_from_node()");

    let metadata_qrs = find_metadata_qrs(&config.qr_dir)?;
    let specs_qrs = find_spec_qrs(&config.qr_dir)?;

    let mut is_changed = false;
    for chain in config.chains {
        // if !specs_qrs.contains_key(chain.name.as_str()) {
        //     let specs = fetcher.fetch_specs(&chain)?;
        //      (&specs, &config.qr_dir)?;
        //     is_changed = true;
        // }
        let specs = fetcher.fetch_specs(&chain)?;
        // (&specs, &config.qr_dir)?;
        is_changed = true;

        println!("chain={}", chain.name.as_str());

        // let secret = "0xc8fa03532fb22ee1f7f6908b9c02b4e72483f0dbd66e4cd456b8f34c6230b849";
        let secret = "caution juice atom organ advance problem want pledge someone senior holiday very";

        let sr25519_pair =
        sr25519::Pair::from_string(secret, None).ok();
        match sr25519_pair {
            Some(pair) => {
                println!("pair={}", pair.public().to_string());
                let signature = pair.sign(&specs[..]).0.to_vec();
                println!("signature={}", signature.encode_hex());
         },
            None => {},
        }


//        println!("sr25519_pair={}", sr25519_pair);


        // let fetched_meta = fetcher.fetch_metadata(&chain)?;
        // let version = fetched_meta.meta_values.version;

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

