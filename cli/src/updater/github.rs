use crate::config::GithubRepo;
use crate::updater::wasm::WasmRuntime;

// fetch the latest chain runtime from GitHub
pub(crate) async fn fetch_latest_runtime(
    config: &GithubRepo,
    chain: &str,
) -> anyhow::Result<Option<WasmRuntime>> {
    log::debug!("fetch_latest_runtime()");

    let release = octocrab::instance()
        .repos(&config.owner, &config.repo)
        .releases()
        .get_latest()
        .await?;
    for asset in release.assets {
        if let Ok(wasm) = WasmRuntime::try_from(asset) {
            log::debug!("wasm.chain={} chain={}", wasm.chain, chain);
            if wasm.chain == chain {
                return Ok(Some(wasm));
            }
        }
    }
    Ok(None)
}
