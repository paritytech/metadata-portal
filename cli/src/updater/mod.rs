mod export;
mod metadata;
mod qr_file;

use crate::config::AppConfig;
use crate::lib::read::{metadata_qr_in_dir, specs_qr_in_dir};

use crate::updater::metadata::fetch_chain_info;
use crate::updater::qr_file::{generate_metadata_qr, generate_spec_qr};

pub fn update(config: AppConfig) -> anyhow::Result<()> {
    let metadata_qrs = metadata_qr_in_dir(&config.qr_dir)?;
    let specs_qrs = specs_qr_in_dir(&config.qr_dir)?;

    let mut is_changed = false;
    for chain in config.chains {
        let meta_specs = fetch_chain_info(
            &chain.rpc_endpoint,
            &chain.token_unit,
            &chain.token_decimals,
        )?;
        if !specs_qrs.contains_key(chain.name.as_str()) {
            generate_spec_qr(&meta_specs, &config.qr_dir)?;
            is_changed = true;
        }
        match metadata_qrs.get(chain.name.as_str()) {
            Some((_, version)) if *version >= meta_specs.meta_values.version => (),
            _ => {
                generate_metadata_qr(&meta_specs, &config.qr_dir)?;
                is_changed = true;
            }
        };
    }

    if !is_changed {
        println!("Everything is up to date!");
        return Ok(());
    }

    println!("Done!");
    Ok(())
}
