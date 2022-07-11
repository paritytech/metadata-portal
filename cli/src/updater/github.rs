use std::collections::HashMap;

use log::info;

use crate::config::GitHub;
use crate::lib::types::ChainName;
use crate::updater::wasm::WasmRuntime;

// fetch latest runtimes from Parity GitHub
pub(crate) async fn fetch_release_runtimes(
    config: &GitHub,
) -> anyhow::Result<HashMap<ChainName, WasmRuntime>> {
    let mut runtimes: HashMap<ChainName, WasmRuntime> = HashMap::new();
    let release = octocrab::instance()
        .repos(&config.owner, &config.repo)
        .releases()
        .get_latest()
        .await?;
    info!("📅 Latest release: {}", release.tag_name);
    for asset in release.assets {
        if let Ok(wasm) = WasmRuntime::try_from(asset) {
            runtimes.insert(wasm.chain.clone(), wasm);
        }
    }
    Ok(runtimes)
}
