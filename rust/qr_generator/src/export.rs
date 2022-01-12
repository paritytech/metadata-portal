use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

use definitions::{crypto::Encryption, metadata::MetaValues};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReactAssetPath(String);

impl ReactAssetPath {
    pub fn from_fs_path(path: PathBuf, public_dir: &PathBuf) -> anyhow::Result<ReactAssetPath> {
        Ok(ReactAssetPath(format!("/{}", path.strip_prefix(public_dir).unwrap().to_str().unwrap())))
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
    pub color: String,
    pub decimals: u8,
    pub genesis_hash: String,
    pub logo: String,
    pub name: String,
    pub secondary_color: String,
    pub unit: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExportChainSpec {
    pub name: String,
    pub icon_path: String,
    pub rpc_endpoint: String,
    pub genesis_hash: String,
    pub color: String,
    pub unit: String,
    pub address_prefix: String,

    pub metadata_qr_codes: Vec<QrCode>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QrCode {
    pub path: ReactAssetPath,
    pub is_verified: bool,
    pub version: u32,
}
