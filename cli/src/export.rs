use std::path::Path;
use std::{fmt, fs};

use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::common::path::QrPath;
use crate::common::types::ChainPortalId;
use crate::source::{read_png_source, Source};
use crate::AppConfig;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub(crate) struct ReactAssetPath(String);

impl ReactAssetPath {
    pub(crate) fn from_fs_path(path: &Path, public_dir: &Path) -> Result<ReactAssetPath> {
        Ok(ReactAssetPath(
            path.to_path_buf()
                .strip_prefix(public_dir)?
                .to_str()
                .unwrap()
                .to_owned(),
        ))
    }
}

impl fmt::Display for ReactAssetPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ExportChainSpec {
    pub(crate) title: String,
    pub(crate) color: String,
    pub(crate) rpc_endpoint: String,

    pub(crate) genesis_hash: String,
    pub(crate) unit: String,
    pub(crate) base58prefix: u16,
    pub(crate) logo: String,
    pub(crate) decimals: u8,

    pub(crate) live_meta_version: u32,
    pub(crate) metadata_qr: Option<MetadataQr>,
    pub(crate) latest_metadata: ReactAssetPath,
    pub(crate) specs_qr: QrCode,
    pub(crate) relay_chain: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MetadataQr {
    pub(crate) version: u32,
    pub(crate) file: QrCode,
}

pub(crate) type ExportData = IndexMap<ChainPortalId, ExportChainSpec>;

pub(crate) fn read_export_file(config: &AppConfig) -> Result<ExportData> {
    let chain_specs =
        fs::read_to_string(&config.data_file).context(format!("{}", config.data_file.display()))?;
    Ok(serde_json::from_str(&chain_specs)?)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QrCode {
    pub(crate) path: ReactAssetPath,
    pub(crate) signed_by: Option<String>,
    pub(crate) source: Option<Source>,
}

impl QrCode {
    pub(crate) fn from_qr_path(config: &AppConfig, qr_path: QrPath) -> Result<QrCode> {
        let path = ReactAssetPath::from_fs_path(&qr_path.to_path_buf(), &config.public_dir)?;
        let signed_by = match qr_path.file_name.is_signed {
            true => Some(config.verifier.name.clone()),
            false => None,
        };
        let source = read_png_source(&qr_path.to_path_buf())?;
        Ok(QrCode {
            path,
            signed_by,
            source,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn create_react_asset_path() {
        let img_path = Path::new("./../public/qr/name_kind_9123.apng").to_path_buf();
        let public_dir = Path::new("./../public").to_path_buf();
        assert_eq!(
            ReactAssetPath::from_fs_path(&img_path, &public_dir).unwrap(),
            ReactAssetPath("qr/name_kind_9123.apng".to_string())
        );
    }
}
