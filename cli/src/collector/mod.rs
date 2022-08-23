pub(crate) mod export;
mod file;

use std::process::exit;

use log::warn;

use crate::collector::export::export_specs;
use crate::collector::file::save_to_file;
use crate::config::AppConfig;
use crate::export::ExportData;
use crate::fetch::RpcFetcher;

pub(crate) fn collect(config: AppConfig) -> anyhow::Result<()> {
    let specs: ExportData = export_specs(&config, RpcFetcher)?;
    save_to_file(&specs, config.data_file)?;
    if specs.len() != config.chains.len() {
        warn!("⚠️ Metadata has to be updated!");
        exit(12);
    }
    Ok(())
}
