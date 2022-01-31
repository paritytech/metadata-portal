use std::fs;
use std::path::{PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub data_file_path: PathBuf,
    pub public_dir_path: PathBuf,
    pub public: PublicDir,
    pub verifier: Verifier,
    pub chains: Vec<Chain>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicDir {
    pub signed_qr_dir: PathBuf,
    pub unsigned_qr_dir: PathBuf,
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
    pub color: String,
}

pub fn read_app_config(config_file: PathBuf) -> anyhow::Result<AppConfig> {
    let config_toml = fs::read_to_string(&config_file)?;
    let mut config = toml::from_str::<AppConfig>(config_toml.as_str())?;

    let root = config_file.parent().unwrap();
    config.public_dir_path = root.join(config.public_dir_path);
    config.data_file_path = root.join(config.data_file_path);
    config.public.signed_qr_dir = config.public_dir_path.join(config.public.signed_qr_dir);
    config.public.unsigned_qr_dir = config.public_dir_path.join(config.public.unsigned_qr_dir);
    Ok(config)
}
