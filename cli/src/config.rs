use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub data_file: PathBuf,
    pub public_dir: PathBuf,
    pub qr_dir: PathBuf,
    pub verifier: Verifier,
    pub chains: Vec<Chain>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Verifier {
    pub name: String,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chain {
    pub name: String,
    pub rpc_endpoint: String,
    pub token_unit: Option<String>,
    pub token_decimals: Option<u8>,
}

pub fn read_app_config(config_file: PathBuf) -> anyhow::Result<AppConfig> {
    let config_toml = fs::read_to_string(&config_file)?;
    let mut config = toml::from_str::<AppConfig>(config_toml.as_str())?;

    let abs_config_path = fs::canonicalize(config_file)?;
    let root = abs_config_path.parent().unwrap();
    config.public_dir = root.join(config.public_dir);
    config.data_file = root.join(config.data_file);
    config.qr_dir = root.join(config.qr_dir);
    Ok(config)
}
