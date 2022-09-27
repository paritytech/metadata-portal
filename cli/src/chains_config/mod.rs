use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::config::{Chain, Verifier};
use crate::opts::ChainsOpts;
use crate::AppConfig;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct ConfigTemplate {
    pub(crate) data_file: Option<PathBuf>,
    pub(crate) public_dir: PathBuf,
    pub(crate) qr_dir: PathBuf,
    pub(crate) verifier: Verifier,
    pub(crate) chains: HashMap<String, ChainTemplate>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct ChainTemplate {
    pub(crate) name: String,
    pub(crate) color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ChainJSON {
    pub(crate) name: String,
    pub(crate) nodes: Vec<ChainNode>,
    pub(crate) icon: String,
    pub(crate) options: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ChainNode {
    pub(crate) name: String,
    pub(crate) url: String,
}

const EXCLUDE_CHAINS: [&str; 7] = [
    "Polkadot",
    "Kusama",
    "Westend",
    "Moonbeam",
    "Moonriver",
    "Moonbase Relay Testnet",
    "Arctic Relay Testnet",
];

pub(crate) fn update_chains_config(chains_opts: ChainsOpts) -> Result<()> {
    let template_path = Path::new("config-template.toml");
    let config_template_toml = fs::read_to_string(&template_path)?;
    let config_template = toml::from_str::<ConfigTemplate>(config_template_toml.as_str())?;

    let chain_params = match chains_opts.env.as_str() {
        "dev" => (
            "config_dev.toml",
            "public/data_dev.json",
            format!(
                "https://raw.githubusercontent.com/nova-wallet/nova-utils/master/chains/{}/{}",
                chains_opts.version, "chains_dev.json"
            ),
        ),
        "prod" => (
            "config.toml",
            "public/data.json",
            format!(
                "https://raw.githubusercontent.com/nova-wallet/nova-utils/master/chains/{}/{}",
                chains_opts.version, "chains.json"
            ),
        ),
        _ => bail!("Unknown env. Should be dev or prod"),
    };

    let chains_json_response = reqwest::blocking::get(chain_params.2).unwrap();
    let chains_json: Vec<ChainJSON> = chains_json_response.json().unwrap();

    let mut chains: Vec<Chain> = vec![];
    for chain in chains_json {
        if EXCLUDE_CHAINS.contains(&chain.name.as_str()) {
            continue;
        }
        let chain_template = config_template.chains.get(&chain.name);
        match chain_template {
            Some(chain_template) => {
                chains.push(Chain {
                    name: String::from(&chain_template.name),
                    title: Some(chain.name),
                    color: String::from(&chain_template.color),
                    icon: chain.icon,
                    rpc_endpoints: chain.nodes.iter().map(|node| node.url.clone()).collect(),
                    github_release: None,
                    token_decimals: None,
                    token_unit: None,
                    testnet: match chain.options {
                        Some(options) => Some(options.contains(&String::from("testnet"))),
                        None => Some(false),
                    },
                });
            }
            None => bail!("No chain {} found!", chain.name),
        }
    }

    let new_config = AppConfig {
        data_file: PathBuf::from(chain_params.1),
        public_dir: config_template.public_dir,
        qr_dir: config_template.qr_dir,
        verifier: config_template.verifier,
        chains,
    };
    let saved = new_config.save(Path::new(chain_params.0));
    if saved.is_err() {
        return Err(saved.err().unwrap());
    }

    Ok(())
}
