use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use anyhow;

mod metadata;
use crate::config::AppConfig;
use crate::Error::UnexpectedQrFilename;

mod error;
pub mod config;
mod export;
mod qr_file;

use crate::error::Error;
use crate::export::{ExportChainSpec, QrCode, ReactAssetPath};
use crate::metadata::fetch_chain_info;
use crate::qr_file::{generate_metadata_qr, read_qr_dir};


pub fn full_run(app_config: AppConfig) -> anyhow::Result<()> {
    let saved_qr_codes = saved_qr_codes(&app_config)?;

    let mut specs = vec![];
    for chain in app_config.chains {
        let meta_specs = fetch_chain_info(&chain.rpc_endpoint)?;
        let metadata_qr_codes = match saved_qr_codes.get(chain.name.as_str()) {
            Some(newest_qr) if newest_qr.version < meta_specs.meta_values.version => {
                let png_path = generate_metadata_qr(&meta_specs.meta_values, &app_config.public.unsigned_qr_dir)?;
                vec![QrCode{
                    path: ReactAssetPath::from_fs_path(png_path, &app_config.public_dir_path)?,
                    is_verified: false,
                    version: meta_specs.meta_values.version
                }]
            },
            Some(qr) => vec![qr.clone()],
            None => vec![],
        };
        specs.push(ExportChainSpec {
            name: chain.name,
            rpc_endpoint: chain.rpc_endpoint,
            genesis_hash: meta_specs.specs.genesis_hash,
            color: chain.color,
            unit: meta_specs.specs.unit,
            address_prefix: meta_specs.specs.base58prefix.to_string(),
            metadata_qr_codes,
        });
    }

    let serialized = serde_json::to_string_pretty(&specs).unwrap();
    let mut file = File::create(app_config.data_file_path)?;
    file.write_all(&serialized.as_bytes())?;
    Ok(())
}

fn saved_qr_codes(config: &AppConfig) -> anyhow::Result<HashMap<String, QrCode>> {
    let mut qrs = HashMap::new();
    read_qr_dir(&mut qrs, &config.public.signed_qr_dir, &config.public_dir_path, true)?;
    read_qr_dir(&mut qrs, &config.public.unsigned_qr_dir, &config.public_dir_path, false)?;
    Ok(qrs)
}
