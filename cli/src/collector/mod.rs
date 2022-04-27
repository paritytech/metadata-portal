mod export;

mod metadata;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use crate::collector::export::{ExportChainSpec, QrCode, ReactAssetPath};
use crate::collector::metadata::fetch_chain_info;
use crate::config::AppConfig;
use crate::lib::read::{metadata_qr_in_dir, specs_qr_in_dir};
use anyhow::bail;

pub fn collect(config: AppConfig) -> anyhow::Result<()> {
    let specs = export_specs(&config)?;
    save_to_file(&specs, config.data_file)?;
    Ok(())
}

fn save_to_file(specs: &Vec<ExportChainSpec>, path: impl AsRef<Path>) -> anyhow::Result<()> {
    let serialized = serde_json::to_string_pretty(specs).unwrap();
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;
    file.write_all("\n".as_bytes())?;
    Ok(())
}

fn export_specs(config: &AppConfig) -> anyhow::Result<Vec<ExportChainSpec>> {
    let metadata_qrs_for_chain = metadata_qr_in_dir(&config.qr_dir)?;
    let specs_qrs_for_chain = specs_qr_in_dir(&config.qr_dir)?;

    let mut export_specs = vec![];
    for chain in &config.chains {
        print!("Collecting {} info...", chain.name);
        // ensure the output is emitted immediately
        io::stdout().flush().unwrap();

        let meta_specs = fetch_chain_info(&chain.rpc_endpoint)?;

        let (metadata_qr, metadata_version) = match metadata_qrs_for_chain.get(chain.name.as_str())
        {
            Some((qr_path, version)) => {
                let path = ReactAssetPath::from_fs_path(qr_path.to_path_buf(), &config.public_dir)
                    .unwrap();
                let signed_by = match qr_path.file_name.is_signed {
                    true => Some(config.verifier.name.clone()),
                    false => None,
                };
                (QrCode { path, signed_by }, *version)
            }
            _ => bail!("failed to find metadata qr for {}", chain.name),
        };
        let specs_qr = match specs_qrs_for_chain.get(chain.name.as_str()) {
            Some(qr_path) => {
                let signed_by = match qr_path.file_name.is_signed {
                    true => Some(config.verifier.name.clone()),
                    false => None,
                };
                let path = ReactAssetPath::from_fs_path(qr_path.to_path_buf(), &config.public_dir)
                    .unwrap();
                QrCode { path, signed_by }
            }
            _ => bail!("failed to find specs qr for {}", chain.name),
        };
        export_specs.push(ExportChainSpec {
            name: chain.name.clone(),
            rpc_endpoint: chain.rpc_endpoint.clone(),
            genesis_hash: meta_specs.specs.genesis_hash,
            unit: meta_specs.specs.unit,
            logo: meta_specs.specs.logo,
            decimals: meta_specs.specs.decimals,
            base58prefix: meta_specs.specs.base58prefix.to_string(),
            metadata_qr,
            specs_qr,
            metadata_version,
        });
        println!("Ok!");
    }
    Ok(export_specs)
}
