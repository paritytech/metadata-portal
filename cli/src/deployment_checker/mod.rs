use std::fs;
use std::path::Path;
use std::process::exit;

use anyhow::Result;
use log::{info, warn};
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::collector::export::export_specs;
use crate::export::{ExportData, ReactAssetPath};
use crate::fetch::RpcFetcher;
use crate::AppConfig;

#[derive(Serialize, Deserialize)]
struct PkgJson {
    homepage: String,
}

// Check whether the deployment is up to date.
// Exit code 12 if re-deploy is required
pub(crate) fn check_deployment(config: AppConfig) -> Result<()> {
    log::debug!("check_deployment()");

    let pkg_json = fs::read_to_string(Path::new("package.json"))?;
    let pkg_json: PkgJson = serde_json::from_str(&pkg_json)?;

    let data_file = ReactAssetPath::from_fs_path(&config.data_file, &config.public_dir)?;
    let url = Url::parse(&pkg_json.homepage)?;
    let url = url.join(&data_file.to_string())?;

    let online = reqwest::blocking::get(url)?.json::<ExportData>()?;
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
