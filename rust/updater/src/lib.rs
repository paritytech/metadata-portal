use anyhow;
use app_config::AppConfig;
use qr_lib::read::latest_qr_per_chain;

mod metadata;

mod export;
mod qr_file;

use crate::metadata::fetch_chain_info;
use crate::qr_file::{generate_metadata_qr};


pub fn full_run(config: AppConfig) -> anyhow::Result<()> {
    let saved_qr_codes = latest_qr_per_chain(&config.qr_dir)?;

    let mut to_update = vec![];
    for chain in config.chains {
        let meta_specs = fetch_chain_info(&chain.rpc_endpoint)?;
        match saved_qr_codes.get(chain.name.as_str()) {
            Some(saved) if saved.file_name.version >= meta_specs.meta_values.version => (),
            _ => {
                to_update.push(meta_specs);
            },
        };
    }

    if to_update.is_empty() {
        println!("Everything is up to date!");
        return Ok(())
    }

    for meta_specs in to_update {
        generate_metadata_qr(&meta_specs, &config.qr_dir)?;
    }

    println!("Done!");
    Ok(())
}
