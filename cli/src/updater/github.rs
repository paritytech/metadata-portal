use std::env;

use octocrab::Octocrab;

use crate::config::GithubRepo;
use crate::updater::wasm::WasmRuntime;

// fetch the latest chain runtime from GitHub
pub(crate) async fn fetch_latest_runtime(
    config: &GithubRepo,
    chain: &str,
) -> anyhow::Result<Option<WasmRuntime>> {
    let github = match env::var("GITHUB_TOKEN") {
        Ok(token) => Octocrab::builder().personal_token(token).build()?,
        Err(_) => Octocrab::default(),
    };
    let release = github
        .repos(&config.owner, &config.repo)
        .releases()
        .get_latest()
        .await?;
    for asset in release.assets {
        if let Ok(wasm) = WasmRuntime::try_from(asset) {
            if wasm.chain == chain {
                return Ok(Some(wasm));
            }
        }
    }
    Ok(None)
}
