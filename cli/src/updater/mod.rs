mod generate;
mod github;
pub(crate) mod source;
mod wasm;

use std::str::FromStr;

use anyhow::{bail, Context};
use blake2_rfc::blake2b::blake2b;
use definitions::crypto::SufficientCrypto;
use log::info;
use sp_core::{H256, sr25519, Pair};
use parity_scale_codec::Encode;
use generate_message::parser::{Command as SignerCommand, Crypto, Goal, Make, Msg};

use crate::config::AppConfig;
use crate::fetch::Fetcher;
use crate::lib::camera::read_qr_file;
use crate::lib::path::QrPath;
use crate::qrs::{find_metadata_qrs, find_spec_qrs};
use crate::signer::sign_qr;
use crate::source::{save_source_info, Source};
use crate::updater::generate::{generate_metadata_qr, generate_spec_qr};
use crate::updater::github::fetch_release_runtimes;
use crate::updater::wasm::{download_wasm, meta_values_from_wasm_bytes};

pub(crate) fn update_from_node(config: AppConfig, sign: bool, fetcher: impl Fetcher) -> anyhow::Result<()> {
    let metadata_qrs = find_metadata_qrs(&config.qr_dir)?;
    let specs_qrs = find_spec_qrs(&config.qr_dir)?;

    let mut is_changed = false;
    for chain in config.chains {
        if !specs_qrs.contains_key(chain.name.as_str()) {
            let specs = fetcher.fetch_specs(&chain)?;
            generate_spec_qr(&chain.name, &specs, &config.qr_dir)?;
            is_changed = true;
        }

        let fetched_meta = fetcher.fetch_metadata(&chain)?;
        let version = fetched_meta.meta_values.version;

        // Skip if already have QR for the same version
        if let Some(map) = metadata_qrs.get(&chain.name) {
            if map.contains_key(&version) {
                continue;
            }
        }
        let path = generate_metadata_qr(
            &fetched_meta.meta_values,
            &fetched_meta.genesis_hash,
            &config.qr_dir,
        )?;
        let source = Source::Rpc {
            url: chain.rpc_endpoint,
            block: fetched_meta.block_hash,
        };
        save_source_info(&path, &source)?;
        is_changed = true;
        if sign {
          let unsigned_qr = QrPath::try_from(&path).unwrap();
          println!("{:?}", unsigned_qr);
          let raw_read = read_qr_file(&unsigned_qr.to_path_buf())?;
          let sr25519_pair = match sr25519::Pair::from_string("", None) {
            Ok(x) => x,
            Err(_e) => {bail!("❌ Key error. Sign metadata and specs manually")}
          };
          let signature = sr25519_pair.sign(&raw_read.as_bytes());
          let make = Make {
            goal: Goal::Qr,
            crypto: Crypto::Sufficient(sufficient_crypto),
            msg: msg_type(passed_crypto.message),
            name: Some(signed_qr.to_string()),
          };
          let sc = SufficientCrypto::Sr25519 {
            public: sr25519_pair.public().to_owned(),
            signature,
          };
          let signature2 = hex::encode(sc.encode());
          sign_qr(&unsigned_qr, signature2.as_str())?;
          println!("🎉 Signed!");
        }
    }

    if !is_changed {
        info!("🎉 Everything is up to date!");
    }
    Ok(())
}

#[tokio::main]
pub(crate) async fn update_from_github(config: AppConfig, sign: bool) -> anyhow::Result<()> {
    if config.github.is_none() {
        info!("↪️ No GitHub repository specified, skipping update");
        return Ok(());
    }
    let gh = &config.github.unwrap();
    let runtimes = fetch_release_runtimes(gh).await?;
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
        let genesis_hash = H256::from_str(&genesis_hash).unwrap();

        // Skip if already have QR for the same version
        if let Some(map) = metadata_qrs.get(&chain.name) {
            if map.contains_key(&wasm.version) {
                left_to_update -= 1;
                continue;
            }
        }
        let wasm_bytes = download_wasm(wasm.to_owned()).await?;
        let meta_hash = blake2b(32, &[], &wasm_bytes).as_bytes().to_vec();
        let meta_values = meta_values_from_wasm_bytes(&wasm_bytes)?;
        let path = generate_metadata_qr(&meta_values, &genesis_hash, &config.qr_dir)?;
        let source = Source::Wasm {
            github_repo: format!("{}/{}", gh.owner, gh.repo),
            hash: format!("0x{}", hex::encode(meta_hash)),
        };
        save_source_info(&path, &source)?;
        if sign {
          println!("sign!");
        }
    }
    if left_to_update == 0 {
        info!("🎉 Everything is up to date!");
    }
    Ok(())
}
