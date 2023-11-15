use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use anyhow::{Context, Result};
use indexmap::IndexMap;
use log::{info, warn};

use crate::common::path::{ContentType, QrPath};
use crate::common::types::MetaVersion;
use crate::export::{ExportChainSpec, ExportData, MetadataQr, QrCode, ReactAssetPath};
use crate::fetch::{fetch_deployed_data, Fetcher};
use crate::qrs::{collect_metadata_qrs, metadata_files, spec_files};
use crate::AppConfig;

pub(crate) fn export_specs(config: &AppConfig, fetcher: impl Fetcher) -> Result<ExportData> {
    let all_specs = spec_files(&config.qr_dir)?;
    let all_metadata = metadata_files(&config.qr_dir)?;
    let online = fetch_deployed_data(config).ok();

    let mut export_specs = IndexMap::new();
    for chain in &config.chains {
        info!("Collecting {} info...", chain.name);
        let specs = match fetcher.fetch_specs(chain) {
            Ok(specs) => specs,
            Err(e) => {
                if let Some(online_specs) = online.as_ref() {
                    if let Some(online_chain_specs) = online_specs.get(&chain.portal_id()) {
                        warn!(
                            "Unable to fetch specs for {}. Keep current online specs. Err: {}.",
                            chain.name, e
                        );
                        export_specs.insert(chain.portal_id(), online_chain_specs.clone());
                        continue;
                    }
                }
                return Err(e);
            }
        };
        let meta = fetcher.fetch_metadata(chain)?;
        let live_meta_version = meta.meta_values.version;

        let metadata_qrs =
            collect_metadata_qrs(&all_metadata, &chain.portal_id(), &live_meta_version)?;

        let specs_qr = all_specs
            .get(&chain.portal_id())
            .with_context(|| format!("No specs qr found for {}", chain.portal_id()))?
            .clone();
        let pointer_to_latest_meta = update_pointer_to_latest_metadata(
            metadata_qrs
                .first()
                .context(format!("No metadata QRs for {}", &chain.name))?,
        )?;
        export_specs.insert(
            chain.portal_id(),
            ExportChainSpec {
                title: chain.formatted_title(),
                color: chain.color.clone(),
                rpc_endpoint: chain.rpc_endpoints[0].clone(), // keep only the first one
                genesis_hash: format!("0x{}", hex::encode(specs.genesis_hash)),
                unit: specs.unit,
                logo: specs.logo,
                decimals: specs.decimals,
                base58prefix: specs.base58prefix,
                specs_qr: QrCode::from_qr_path(config, specs_qr)?,
                latest_metadata: ReactAssetPath::from_fs_path(
                    &pointer_to_latest_meta,
                    &config.public_dir,
                )?,
                metadata_qr: export_live_metadata(config, metadata_qrs, &live_meta_version),
                live_meta_version,
                relay_chain: chain.relay_chain.clone(),
            },
        );
    }
    Ok(export_specs)
}

fn export_live_metadata(
    config: &AppConfig,
    qrs: Vec<QrPath>,
    live_version: &MetaVersion,
) -> Option<MetadataQr> {
    qrs.into_iter()
        .find(
            |qr| matches!(qr.file_name.content_type, ContentType::Metadata(v) if v==*live_version),
        )
        .map(|qr| MetadataQr {
            version: *live_version,
            file: QrCode::from_qr_path(config, qr).unwrap(),
        })
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
    symlink(metadata_qr.to_path_buf(), &latest_metadata_qr).unwrap();
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
