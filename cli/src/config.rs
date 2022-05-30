use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use log::debug;
use std::fs;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct AppConfig {
    pub(crate) data_file: PathBuf,
    pub(crate) public_dir: PathBuf,
    pub(crate) qr_dir: PathBuf,
    pub(crate) verifier: Verifier,
    pub(crate) github: Option<GitHub>,
    pub(crate) chains: Vec<Chain>,
}

#[cfg(test)]
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            data_file: PathBuf::from("data.json"),
            public_dir: PathBuf::from("src/public"),
            qr_dir: PathBuf::from("qr"),
            verifier: Verifier::default(),
            github: None,
            chains: vec![Chain::default()],
        }
    }
}

impl AppConfig {
    pub(crate) fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let abs_config_path = fs::canonicalize(&path)?;
        debug!("Loading config from: {}", abs_config_path.display());
        let root = abs_config_path.parent().unwrap();

        let config_toml = fs::read_to_string(&path)?;
        let mut config = toml::from_str::<AppConfig>(config_toml.as_str())?;

        config.public_dir = root.join(config.public_dir);
        config.data_file = root.join(config.data_file);
        config.qr_dir = root.join(config.qr_dir);
        Ok(config)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct Verifier {
    pub(crate) name: String,
    pub(crate) public_key: String,
}

#[cfg(test)]
impl Default for Verifier {
    fn default() -> Self {
        Self {
            name: "Test Verifier".to_string(),
            public_key: "123".to_string(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct GitHub {
    pub(crate) owner: String,
    pub(crate) repo: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct Chain {
    pub(crate) name: String,
    pub(crate) color: String,
    pub(crate) rpc_endpoint: String,
    pub(crate) color: String,
    pub(crate) genesis_hash: Option<String>,
}

#[cfg(test)]
impl Default for Chain {
    fn default() -> Self {
        Self {
            name: "polkadot".to_string(),
            color: "#e6007a".to_string(),
            rpc_endpoint: "wss://example.com".to_string(),
            genesis_hash: None,
        }
    }
}
