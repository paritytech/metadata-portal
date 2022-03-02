use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use anyhow;
use app_config::{AppConfig};
use qr_lib::read::latest_qr_per_chain;

mod metadata;

mod export;

use crate::export::{ExportChainSpec, QrCode, ReactAssetPath};
use crate::metadata::fetch_chain_info;


pub fn full_run(config: AppConfig) -> anyhow::Result<()> {
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


fn export_specs(config: &AppConfig) ->  anyhow::Result<Vec<ExportChainSpec>> {
    let saved_qr_codes = latest_qr_per_chain(&config.qr_dir)?;

    let mut specs = vec![];
    for chain in &config.chains {
        print!("Collecting {} info...", chain.name);
        // ensure the output is emitted immediately
        io::stdout().flush().unwrap();

        let meta_specs = fetch_chain_info(&chain.rpc_endpoint)?;

        let qr_code = match saved_qr_codes.get(chain.name.as_str()) {
            Some(qr_path) => {
                let signed_by = match qr_path.file_name.is_signed {
                    true => Some(config.verifier.name.clone()),
                    false => None
                };
                Some(QrCode{
                    path: ReactAssetPath::from_fs_path(qr_path.to_path_buf(), &config.public_dir).unwrap(),
                    signed_by,
                    version: qr_path.file_name.version
                })
            },
            _ => None
        };
        specs.push(ExportChainSpec {
            name: chain.name.clone(),
            rpc_endpoint: chain.rpc_endpoint.clone(),
            genesis_hash: meta_specs.specs.genesis_hash,
            color: chain.color.clone(),
            unit: meta_specs.specs.unit,
            address_prefix: meta_specs.specs.base58prefix.to_string(),
            metadata_qr: qr_code,
            add_to_signer: chain.add_to_signer,
        });
        println!("Ok!");
    }
    Ok(specs)
}
