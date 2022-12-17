use std::path::Path;

use anyhow::{anyhow, ensure};
use definitions::metadata::MetaValues;
use log::info;
use octocrab::models::repos::Asset;
use reqwest::Url;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub(crate) struct WasmRuntime {
    pub(crate) chain: String,
    pub(crate) version: u32,
    pub(crate) download_url: Url,
}

impl TryFrom<Asset> for WasmRuntime {
    type Error = anyhow::Error;

    fn try_from(asset: Asset) -> Result<Self, Self::Error> {
        if !asset.name.ends_with(".wasm") {
            return Err(anyhow!("{} has no .wasm extension", asset.name));
        }
        let runtime_info = asset
            .name
            .split('.')
            .next()
            .ok_or_else(|| anyhow!("no runtime info found"))?;
        let mut split = runtime_info.split("_runtime-v");
        let chain = split.next().ok_or_else(|| anyhow!("no chain name found"))?;
        let version: u32 = split
            .next()
            .ok_or_else(|| anyhow!("no metadata version found"))?
            .parse()
            .unwrap();

        Ok(Self {
            chain: String::from(chain),
            version,
            download_url: asset.browser_download_url,
        })
    }
}

pub(crate) async fn download_wasm(wasm: WasmRuntime) -> anyhow::Result<Vec<u8>> {
    info!("⬇️  Downloading {} runtime...", &wasm.chain);
    let response = reqwest::get(wasm.download_url.clone()).await?;
    ensure!(
        response.status().is_success(),
        "failed to download {}: {}",
        wasm.download_url,
        response.status()
    );
    Ok(response.bytes().await?.to_vec())
}

pub(crate) fn meta_values_from_wasm_bytes(wasm_bytes: &Vec<u8>) -> anyhow::Result<MetaValues> {
    let filename = "/tmp/wasm";
    std::fs::write(Path::new(&filename), wasm_bytes)?;
    let meta = MetaValues::from_wasm_file(filename)
        .map_err(|_e| anyhow!("error converting wasm to metadata"))?;
    Ok(meta)
}
