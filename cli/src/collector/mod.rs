pub(crate) mod export;
mod file;

use anyhow::bail;

use crate::collector::export::export_specs;
use crate::collector::file::save_to_file;
use crate::config::AppConfig;
use crate::export::ExportData;
use crate::fetch::RpcFetcher;

pub(crate) fn collect(config: AppConfig) -> anyhow::Result<()> {
    let specs: ExportData = export_specs(&config, RpcFetcher)?;
    save_to_file(&specs, config.data_file)?;
    if specs.len() != config.chains.len() {
        bail!("Metadata has to be updated!");
    }
    Ok(())
}
