use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use anyhow::{Context, Result};
use indexmap::IndexMap;
use log::{info, warn};

use crate::export::{ExportChainSpec, ExportData, QrCode, ReactAssetPath};
use crate::fetch::Fetcher;
use crate::qrs::{extract_metadata_qr, find_metadata_qrs, find_spec_qrs, next_metadata_version};
use crate::utils::path::QrPath;
use crate::AppConfig;

pub(crate) fn export_specs(config: &AppConfig, fetcher: impl Fetcher) -> Result<ExportData> {
    let specs_qrs = find_spec_qrs(&config.qr_dir)?;
    let metadata_qrs = find_metadata_qrs(&config.qr_dir)?;

    let mut export_specs = IndexMap::new();
    for chain in &config.chains {
        info!("Collecting {} info...", chain.name);

        let specs_result = fetcher.fetch_specs(chain);
        let meta_result = fetcher.fetch_metadata(chain);
        if specs_result.is_err() || meta_result.is_err() {
            warn!("Error getting data for {}", chain.name);
            continue;
        }
        let specs = specs_result.unwrap();
        let meta = meta_result.unwrap();
        let active_version = meta.meta_values.version;
        let metadata_qr_result = extract_metadata_qr(&metadata_qrs, &chain.name, &active_version);
        if metadata_qr_result.is_err() {
            warn!("No latest metadata found for {}", chain.name);
            continue;
        }
        let metadata_qr = metadata_qr_result.unwrap();
        let specs_qr = specs_qrs
            .get(chain.name.as_str())
            .with_context(|| format!("No specs qr found for {}", chain.name))?
            .clone();
        let next_version = next_metadata_version(&metadata_qrs, &chain.name, active_version)?;

        let next_metadata_qr = next_version
            .map(|v| extract_metadata_qr(&metadata_qrs, &chain.name, &v).unwrap())
            .map(|qr| QrCode::from_qr_path(config, qr, &chain.verifier).unwrap());
        let latest_meta = update_pointer_to_latest_metadata(&metadata_qr)?;
        export_specs.insert(
            chain.name.clone(),
            ExportChainSpec {
                title: chain.title.as_ref().unwrap_or(&chain.name).clone(),
                color: chain.color.clone(),
                rpc_endpoint: chain.rpc_endpoints[0].clone(), // keep only the first one
                genesis_hash: format!("0x{}", hex::encode(specs.genesis_hash)),
                unit: specs.unit,
                icon: chain.icon.clone(),
                decimals: specs.decimals,
                base58prefix: specs.base58prefix,
                metadata_qr: QrCode::from_qr_path(config, metadata_qr, &chain.verifier)?,
                specs_qr: QrCode::from_qr_path(config, specs_qr, &chain.verifier)?,
                next_metadata_version: next_version,
                next_metadata_qr,
                metadata_version: active_version,
                latest_metadata: ReactAssetPath::from_fs_path(&latest_meta, &config.public_dir)?,
                testnet: chain.testnet.unwrap_or(false),
            },
        );
    }
    Ok(export_specs)
}

// Create symlink to latest metadata qr
fn update_pointer_to_latest_metadata(metadata_qr: &QrPath) -> Result<PathBuf> {
    let latest_metadata_qr = metadata_qr.dir.join(format!(
        "{}_metadata_latest.apng",
        metadata_qr.file_name.chain
    ));
    if latest_metadata_qr.is_symlink() {
        fs::remove_file(&latest_metadata_qr).unwrap();
    }
    symlink(&metadata_qr.to_path_buf(), &latest_metadata_qr).unwrap();
    Ok(latest_metadata_qr)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::{env, fs};

    use definitions::crypto::Encryption;
    use definitions::metadata::MetaValues;
    use definitions::network_specs::NetworkSpecs;
    use generate_message::helpers::MetaFetched;
    use sp_core::H256;

    use super::*;
    use crate::config::Chain;

    struct MockFetcher;
    impl Fetcher for MockFetcher {
        fn fetch_specs(&self, _chain: &Chain) -> Result<NetworkSpecs> {
            Ok(NetworkSpecs {
                base58prefix: 0,
                color: "".to_string(),
                decimals: 10,
                encryption: Encryption::Ed25519,
                genesis_hash: H256::from_str(
                    "a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd",
                )
                .expect("checked value"),
                logo: "logo".to_string(),
                name: "polkadot".to_string(),
                path_id: "".to_string(),
                secondary_color: "".to_string(),
                title: "".to_string(),
                unit: "DOT".to_string(),
            })
        }

        fn fetch_metadata(&self, _chain: &Chain) -> Result<MetaFetched> {
            Ok(MetaFetched {
                meta_values: MetaValues {
                    name: "".to_string(),
                    version: 9,
                    optional_base58prefix: None,
                    warn_incomplete_extensions: false,
                    meta: vec![],
                },
                block_hash: H256::zero(),
                genesis_hash: H256::zero(),
            })
        }
    }

    #[test]
    fn test_collector() {
        let root_dir = env::current_dir().unwrap();
        let config = AppConfig {
            qr_dir: root_dir.join("src/collector/for_tests"),
            public_dir: root_dir.join("src/collector"),
            ..Default::default()
        };

        let specs = export_specs(&config, MockFetcher).unwrap();
        let result = serde_json::to_string_pretty(&specs).unwrap();
        let expected = fs::read_to_string(config.qr_dir.join("expected.json"))
            .expect("unable to read expected file");
        assert_eq!(result, expected);

        let latest_symlink = config.qr_dir.join("polkadot_metadata_latest.apng");
        assert!(latest_symlink.exists());
        fs::remove_file(latest_symlink).unwrap();
    }
}
