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
    pub(crate) verifiers: HashMap<String, Verifier>,
    pub(crate) chains: HashMap<String, ChainTemplate>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct ChainTemplate {
    pub(crate) name: String,
    pub(crate) color: String,
    pub(crate) verifier: Option<String>,
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

const EXCLUDE_CHAINS: [&str; 9] = [
    "Arctic Relay Testnet",
    "Aleph Zero Testnet", //TODO name matches with mainnet and will override it
    "Edgeware",           //TODO (MetadataError(NoVersionInConstants))
    "KICO",               //TODO Specs(Base58PrefixMismatch { specs: 51, meta: 42 })
    "Composable Finance", //TODO  Specs(Base58PrefixMismatch { specs: 50, meta: 49 })
    "Moonbeam",           //uncomment after PS release
    "Moonriver",          //uncomment after PS release
    "Moonbase Relay Testnet", //uncomment after PS release
    "Moonbase Alpha",     //uncomment after PS release
];

pub(crate) fn update_chains_config(chains_opts: ChainsOpts) -> Result<()> {
    let template_path = Path::new("config-template.toml");
    let config_template_toml = fs::read_to_string(template_path)?;
    let config_template = toml::from_str::<ConfigTemplate>(config_template_toml.as_str())?;

    let chain_params = match chains_opts.env.as_str() {
        "dev" => (
            "config_dev.toml",
            "public/data_dev.json",
            format!(
                "https://raw.githubusercontent.com/nova-wallet/nova-utils/master/chains/{}/{}",
                chains_opts.version, "chains_dev.json"
            ),
            "public/qr_dev",
        ),
        "prod" => (
            "config.toml",
            "public/data.json",
            format!(
                "https://raw.githubusercontent.com/nova-wallet/nova-utils/master/chains/{}/{}",
                chains_opts.version, "chains.json"
            ),
            "public/qr",
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
                    testnet: match &chain.options {
                        Some(options) => Some(options.contains(&String::from("testnet"))),
                        None => Some(false),
                    },
                    verifier: match &chain_template.verifier {
                        Some(value) => String::from(value),
                        None => String::from("novasama"),
                    },
                    encryption: match &chain.options {
                        Some(options) => {
                            if options.contains(&String::from("ethereumBased")) {
                                Some(String::from("ethereum"))
                            } else {
                                None
                            }
                        }
                        None => None,
                    },
                });
            }
            None => bail!("No chain {} found!", chain.name),
        }
    }

    let new_config = AppConfig {
        data_file: PathBuf::from(chain_params.1),
        public_dir: config_template.public_dir,
        qr_dir: PathBuf::from(chain_params.3),
        verifiers: config_template.verifiers,
        chains,
    };
    let saved = new_config.save(Path::new(chain_params.0));
    if saved.is_err() {
        return Err(saved.err().unwrap());
    }

    Ok(())
}
