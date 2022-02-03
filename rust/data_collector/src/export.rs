use std::path::{PathBuf};
use serde::{Serialize, Deserialize};

use definitions::{metadata::MetaValues};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReactAssetPath(String);

impl ReactAssetPath {
    pub fn from_fs_path(path: PathBuf, public_dir: &PathBuf) -> anyhow::Result<ReactAssetPath> {
        Ok(ReactAssetPath(format!("/{}", path.strip_prefix(public_dir)?.to_str().unwrap())))
    }
}


/// Struct to store MetaValues, genesis hash, and ChainSpecsToSend for network
pub struct MetaSpecs {
    pub meta_values: MetaValues,
    pub specs: ChainSpecs
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChainSpecs {
    pub base58prefix: u16,
    pub decimals: u8,
    pub genesis_hash: String,
    pub logo: String,
    pub name: String,
    pub unit: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExportChainSpec {
    pub name: String,
    pub rpc_endpoint: String,
    pub genesis_hash: String,
    pub color: String,
    pub unit: String,
    pub address_prefix: String,

    pub metadata_qr: Option<QrCode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QrCode {
    pub path: ReactAssetPath,
    pub signed_by: Option<String>,
    pub version: u32,
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn create_react_asset_path() {
        let img_path = Path::new("./../public/qr/name_kind_9123.apng").to_path_buf();
        let public_dir = Path::new("./../public").to_path_buf();
        assert_eq!(ReactAssetPath::from_fs_path(img_path, &public_dir).unwrap(),
                   ReactAssetPath("/qr/name_kind_9123.apng".to_string()));
    }

}