use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fmt, fs};

use anyhow::bail;
use log::debug;
use serde::de::{self, value, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

fn case_insensitive<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.to_lowercase())
}

fn string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec;

    impl<'de> Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![s.to_owned()])
        }

        fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            Deserialize::deserialize(value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(StringOrVec)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct AppConfig {
    pub(crate) data_file: PathBuf,
    pub(crate) public_dir: PathBuf,
    pub(crate) qr_dir: PathBuf,
    pub(crate) verifiers: HashMap<String, Verifier>,
    pub(crate) chains: Vec<Chain>,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ChainJSON {
    pub(crate) name: String,
    pub(crate) nodes: Vec<ChainNode>,
    pub(crate) icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ChainNode {
    pub(crate) name: String,
    pub(crate) url: String,
}

#[cfg(test)]
impl Default for AppConfig {
    fn default() -> Self {
        let mut verifiers = HashMap::new();
        verifiers.insert(String::from("novasama"), Verifier::default());
        Self {
            data_file: PathBuf::from("data.json"),
            public_dir: PathBuf::from("src/public"),
            qr_dir: PathBuf::from("qr"),
            verifiers,
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

    pub(crate) fn save(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let write_result = fs::write(&path, toml::to_string_pretty(self).unwrap().as_bytes());
        match write_result {
            Ok(_) => println!("File {} was updated!", path.as_ref().to_str().unwrap()),
            Err(e) => bail!("Error saving config.toml! {}", e),
        }

        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct Verifier {
    pub(crate) name: String,
    pub(crate) public_key: String,
    pub(crate) ethereum_public_key: Option<String>,
}

#[cfg(test)]
impl Default for Verifier {
    fn default() -> Self {
        Self {
            name: "Test Verifier".to_string(),
            public_key: "123".to_string(),
            ethereum_public_key: Some("321".to_string()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct GithubRepo {
    pub(crate) owner: String,
    pub(crate) repo: String,
    pub(crate) genesis_hash: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct Chain {
    #[serde(deserialize_with = "case_insensitive")]
    pub(crate) name: String,
    pub(crate) title: Option<String>,
    #[serde(default = "color_default")]
    pub(crate) color: String,
    pub(crate) icon: String,
    #[serde(alias = "rpc_endpoint", deserialize_with = "string_or_vec")]
    pub(crate) rpc_endpoints: Vec<String>,
    pub(crate) token_unit: Option<String>,
    pub(crate) token_decimals: Option<u8>,
    pub(crate) github_release: Option<GithubRepo>,
    pub(crate) testnet: Option<bool>,
    pub(crate) verifier: String,
    pub(crate) encryption: Option<String>,
}

fn color_default() -> String {
    "#000000".to_string()
}

#[cfg(test)]
impl Default for Chain {
    fn default() -> Self {
        Self {
            name: "polkadot".to_string(),
            title: None,
            color: color_default(),
            icon: "Polkadot.svg".to_string(),
            rpc_endpoints: vec!["wss://example.com".to_string()],
            token_unit: None,
            token_decimals: None,
            github_release: None,
            testnet: Some(false),
            verifier: String::from("novasama"),
            encryption: None,
        }
    }
}
