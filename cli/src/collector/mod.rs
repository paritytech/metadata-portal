pub(crate) mod export;
mod file;

use crate::collector::export::export_specs;
use crate::collector::file::save_to_file;
use crate::config::AppConfig;
use crate::fetch::RpcFetcher;

pub(crate) fn collect(config: AppConfig) -> anyhow::Result<()> {
    let specs = export_specs(&config, RpcFetcher)?;
    save_to_file(&specs, config.data_file)?;
    Ok(())
}
