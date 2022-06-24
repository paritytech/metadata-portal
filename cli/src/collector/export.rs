use anyhow::{Context, Result};
use indexmap::IndexMap;
use log::info;

use crate::export::{ExportChainSpec, ExportData, QrCode};
use crate::fetch::Fetcher;
use crate::qrs::{extract_metadata_qr, find_metadata_qrs, find_spec_qrs, next_metadata_version};
use crate::AppConfig;

pub(crate) fn export_specs(config: &AppConfig, fetcher: impl Fetcher) -> Result<ExportData> {
    let specs_qrs = find_spec_qrs(&config.qr_dir)?;
    let metadata_qrs = find_metadata_qrs(&config.qr_dir)?;

    let mut export_specs = IndexMap::new();
    for chain in &config.chains {
        info!("Collecting {} info...", chain.name);

        let specs = fetcher.fetch_specs(chain)?;
        let meta = fetcher.fetch_metadata(chain)?;
        let active_version = meta.meta_values.version;

        let metadata_qr = extract_metadata_qr(&metadata_qrs, &chain.name, &active_version)?;

        let specs_qr = specs_qrs
            .get(chain.name.as_str())
            .with_context(|| format!("No specs qr found for {}", chain.name))?
            .clone();
        let next_version = next_metadata_version(&metadata_qrs, &chain.name, active_version)?;

        let next_metadata_qr = next_version
            .map(|v| extract_metadata_qr(&metadata_qrs, &chain.name, &v).unwrap())
            .map(|qr| QrCode::from_qr_path(config, qr).unwrap());
        export_specs.insert(
            chain.name.clone(),
            ExportChainSpec {
                name: chain.name.clone(),
                color: chain.color.clone(),
                rpc_endpoint: chain.rpc_endpoint.clone(),
                genesis_hash: format!("0x{}", hex::encode(&specs.genesis_hash)),
                unit: specs.unit,
                logo: specs.logo,
                decimals: specs.decimals,
                base58prefix: specs.base58prefix,
                metadata_qr: QrCode::from_qr_path(config, metadata_qr)?,
                specs_qr: QrCode::from_qr_path(config, specs_qr)?,
                next_metadata_version: next_version,
                next_metadata_qr,
                metadata_version: active_version,
            },
        );
    }
    Ok(export_specs)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    use definitions::crypto::Encryption;
    use definitions::metadata::MetaValues;
    use definitions::network_specs::NetworkSpecsToSend;
    use generate_message::helpers::MetaFetched;
    use sp_core::H256;

    use super::*;
    use crate::config::Chain;

    struct MockFetcher;
    impl Fetcher for MockFetcher {
        fn fetch_specs(&self, _chain: &Chain) -> Result<NetworkSpecsToSend> {
            Ok(NetworkSpecsToSend {
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
        let config = AppConfig {
            qr_dir: PathBuf::from("./src/collector/for_tests"),
            public_dir: PathBuf::from("./src/collector"),
            ..Default::default()
        };

        let specs = export_specs(&config, MockFetcher).unwrap();
        let result = serde_json::to_string_pretty(&specs).unwrap();
        let expected = fs::read_to_string(config.qr_dir.join("expected.json"))
            .expect("unable to read expected file");
        assert_eq!(result, expected);
    }
}
