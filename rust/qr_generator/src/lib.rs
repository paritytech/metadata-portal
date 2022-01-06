use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use std::{fs, thread};
use std::path::{Path, PathBuf};
use anyhow;
use definitions::metadata::MetaValues;

mod helpers;
mod metadata_shortcut;
    use metadata_shortcut::meta_shortcut;
use qrcode_rtx::make_pretty_qr;
use crate::config::AppConfig;
use crate::Error::UnexpectedQrFilename;

mod error;
pub mod config;
mod export;
use crate::error::Error;
use crate::export::{ExportChainSpec, QrCode};
use crate::metadata_shortcut::fetch_chain_info;




pub fn full_run(app_config: AppConfig) -> anyhow::Result<()> {
    println!("full_run");

    let saved_qr_codes = saved_qr_codes(&app_config)?;

    let mut specs = vec![];
    for chain in app_config.chains {
        let meta_specs = fetch_chain_info(&chain.rcp_endpoint)?;
        // if let Err(e) = generate_metadata_qr(meta_specs.meta_values) {
        //     eprintln!("Error generating QR for {}: {}", chain.name, e)
        // }

        let metadata_qr_codes = match saved_qr_codes.get(&chain.name) {
            Some(qr) => vec![qr.clone()],
            _ => vec![]
        };

        specs.push(ExportChainSpec {
            name: chain.name,
            icon_path: chain.icon_path,
            rpc_endpoint: chain.rcp_endpoint,
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

fn saved_qr_codes(app_config: &AppConfig) -> anyhow::Result<HashMap<String, QrCode>> {
    let mut qrs = HashMap::new();

    let signed_qr_dir = &app_config.public_dir.join(&app_config.signed_qr_dir);
    read_qr_dir(&mut qrs, signed_qr_dir, &app_config.public_dir, true)?;

    let unsigned_qr_dir = &app_config.public_dir.join(&app_config.unsigned_qr_dir);
    read_qr_dir(&mut qrs, unsigned_qr_dir, &app_config.public_dir, false)?;

    Ok(qrs)
}

fn read_qr_dir(result: &mut HashMap<String, QrCode>, dir: &PathBuf, public_dir: &PathBuf, is_verified: bool) -> anyhow::Result<()>{
    for file in fs::read_dir(dir)? {
        let path = file?.path();
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        let mut split = file_name.split('_');
        match (split.next(), split.next(), split.next()) {
            (Some(chain), Some(data_type), Some(version)) => {

                result.insert(String::from(chain), QrCode{
                    path: path.strip_prefix(public_dir).unwrap().to_path_buf(),
                    is_verified,
                    version: version.parse().unwrap()
                });
            },
            _ => return Err(UnexpectedQrFilename(path).show())
        }
    }
    Ok(())
}

pub fn generate_metadata_qr(meta_values: MetaValues) -> anyhow::Result<()> {
    let crypto_type_code = "ff";
    let prelude = format!("53{}{}", crypto_type_code, "80");
    let output_name= format!("../qr_codes/unsigned/{}_metadata_{}", meta_values.name, meta_values.version);
    println!("generating QR for {}. It takes a while...", meta_values.name);
    let complete_message = [hex::decode(prelude).expect("known value"), meta_values.meta].concat();
    if let Err(e) = make_pretty_qr(&complete_message, &output_name) {
        return Err(Error::Qr(e.to_string()).show())
    }
    Ok(())
}
