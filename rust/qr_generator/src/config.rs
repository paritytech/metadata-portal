use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

const PUBLIC_DIR: &str = "public";

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub data_file_path: PathBuf,
    pub public_dir: PathBuf,
    pub signed_qr_dir: PathBuf,
    pub unsigned_qr_dir: PathBuf,
    pub chains: Vec<Chain>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chain {
    pub name: String,
    pub rcp_endpoint: String,
    pub color: String,
    pub icon_path: String,
}

pub fn read_app_config(config_file: PathBuf) -> anyhow::Result<AppConfig> {
    let config_toml = fs::read_to_string(&config_file)?;
    let mut config = toml::from_str::<AppConfig>(config_toml.as_str())?;

    let root = config_file.parent().unwrap();
    config.public_dir = root.join(config.public_dir);
    config.data_file_path = root.join(config.data_file_path);
    // config.signed_qr_folder = base.join(config.signed_qr_folder);
    // config.unsigned_qr_folder = base.join(config.unsigned_qr_folder);
    Ok(config)
}

