use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub chains: Vec<Chain>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chain {
    pub name: String,
    pub(crate) url: String
}


pub fn read_app_config(path: &str) -> anyhow::Result<AppConfig> {
    let config_toml = fs::read_to_string(path)?;
    let config = toml::from_str::<AppConfig>(config_toml.as_str())?;
    Ok(config)
}
