use std::process::exit;

use anyhow::Result;
use log::{info, warn};

use crate::collector::export::export_specs;
use crate::fetch::{fetch_deployed_data, RpcFetcher};
use crate::AppConfig;

// Check whether the deployment is up to date.
// Exit code 12 if re-deploy is required
pub(crate) fn check_deployment(config: AppConfig) -> Result<()> {
    let online = fetch_deployed_data(&config)?;
    let local = export_specs(&config, RpcFetcher);

    if let Err(e) = local {
        warn!("Error exporting specs: {}", e);
        // Do not re-redeploy because new metadata might not be signed yet
        return Ok(());
    }
    let local = local?;

    if online != local {
        warn!("Re-deploy is required!");
        exit(12)
    }
    info!("🎉 Everything is up to date!");
    Ok(())
}
