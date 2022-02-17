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

    for chain in config.chains {
        let meta_specs = fetch_chain_info(&chain.rpc_endpoint)?;
        match saved_qr_codes.get(chain.name.as_str()) {
            Some(saved) if saved.file_name.version >= meta_specs.meta_values.version => (),
            _ => {
                generate_metadata_qr(&meta_specs, &config.qr_dir)?;
            },
        };
    }
    Ok(())
}
