use anyhow::{bail, Result};
use definitions::crypto::Encryption;
use definitions::network_specs::NetworkSpecsToSend;
use generate_message::helpers::{meta_fetch, specs_agnostic, MetaFetched};
use generate_message::parser::Token;

use crate::config::Chain;

pub(crate) trait Fetcher {
    fn fetch_specs(&self, chain: &Chain) -> Result<NetworkSpecsToSend>;
    fn fetch_metadata(&self, chain: &Chain) -> Result<MetaFetched>;
}

pub(crate) struct RpcFetcher;

impl Fetcher for RpcFetcher {
    fn fetch_specs(&self, chain: &Chain) -> Result<NetworkSpecsToSend> {
        let optional_token_override = chain.token_decimals.zip(chain.token_unit.as_ref()).map(
            |(token_decimals, token_unit)| Token {
                decimals: token_decimals,
                unit: token_unit.to_string(),
            },
        );
        let specs = specs_agnostic(
            &chain.rpc_endpoint,
            Encryption::Sr25519,
            optional_token_override,
            None,
        )
        .map_err(anyhow::Error::msg)?;
        if specs.name.to_lowercase() != chain.name {
            bail!(
                "Network name mismatch. Expected {}, got {}. Please fix it in `config.toml`",
                chain.name,
                specs.name
            )
        }
        Ok(specs)
    }

    fn fetch_metadata(&self, chain: &Chain) -> Result<MetaFetched> {
        let meta = meta_fetch(&chain.rpc_endpoint).map_err(anyhow::Error::msg)?;
        if meta.meta_values.name.to_lowercase() != chain.name {
            bail!(
                "Network name mismatch. Expected {}, got {}. Please fix it in `config.toml`",
                chain.name,
                meta.meta_values.name
            )
        }
        Ok(meta)
    }
}
