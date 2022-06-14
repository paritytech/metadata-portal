use crate::export::{ExportChainSpec, ExportData, QrCode};
use crate::fetch::Fetcher;

use crate::qrs::{extract_metadata_qr, find_metadata_qrs, find_spec_qrs, next_metadata_version};
use crate::AppConfig;
use anyhow::{Context, Result};
use indexmap::IndexMap;
use log::info;

pub(crate) fn export_specs(config: &AppConfig, fetcher: impl Fetcher) -> Result<ExportData> {
    let specs_qrs = find_spec_qrs(&config.qr_dir)?;
    let metadata_qrs = find_metadata_qrs(&config.qr_dir)?;

    let mut export_specs = IndexMap::new();
    for chain in &config.chains {
        info!("Collecting {} info...", chain.name);

        let meta_specs = fetcher.fetch_chain_info(
            &chain.rpc_endpoint,
            &chain.token_unit,
            &chain.token_decimals,
        )?;
        let active_version = meta_specs.meta_values.version;

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
                rpc_endpoint: chain.rpc_endpoint.clone(),
                genesis_hash: meta_specs.specs.genesis_hash,
                unit: meta_specs.specs.unit,
                logo: meta_specs.specs.logo,
                decimals: meta_specs.specs.decimals,
                base58prefix: meta_specs.specs.base58prefix,
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
    use super::*;
    use crate::fetch::{ChainSpecs, MetaSpecs};
    use definitions::metadata::MetaValues;
    use std::fs;
    use std::path::PathBuf;

    struct MockFetcher;
    impl Fetcher for MockFetcher {
        fn fetch_chain_info(
            &self,
            _rpc_endpoint: &str,
            token_unit: &Option<String>,
            token_decimals: &Option<u8>,
        ) -> Result<MetaSpecs> {
            Ok(MetaSpecs {
                specs: ChainSpecs::default(),
                meta_values: MetaValues {
                    name: "".to_string(),
                    version: 9,
                    optional_base58prefix: None,
                    warn_incomplete_extensions: false,
                    meta: vec![],
                },
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
