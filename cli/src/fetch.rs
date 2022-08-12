use anyhow::{bail, Result};
use definitions::crypto::Encryption;
use definitions::error_active::ErrorActive;
use definitions::network_specs::NetworkSpecsToSend;
use generate_message::helpers::{meta_fetch, specs_agnostic, MetaFetched};
use generate_message::parser::Token;
use log::warn;

use crate::config::Chain;

pub(crate) trait Fetcher {
    fn fetch_specs(&self, chain: &Chain) -> Result<NetworkSpecsToSend>;
    fn fetch_metadata(&self, chain: &Chain) -> Result<MetaFetched>;
}

// try to call all urls unless successful
fn call_urls<F, T>(urls: &Vec<String>, f: F) -> Result<T, ErrorActive>
where
    F: Fn(&str) -> Result<T, ErrorActive>,
{
    let n = urls.len();
    for url in urls.iter().take(n - 1) {
        match f(url) {
            Ok(res) => return Ok(res),
            Err(ErrorActive::Fetch(e)) => warn!("Failed to fetch {}: {:?}", url, e),
            Err(e) => return Err(e),
        }
    }
    f(&urls[n - 1])
}

pub(crate) struct RpcFetcher;

impl Fetcher for RpcFetcher {
    fn fetch_specs(&self, chain: &Chain) -> Result<NetworkSpecsToSend> {
        let specs = call_urls(&chain.rpc_endpoints, |url| {
            let optional_token_override = chain.token_decimals.zip(chain.token_unit.as_ref()).map(
                |(token_decimals, token_unit)| Token {
                    decimals: token_decimals,
                    unit: token_unit.to_string(),
                },
            );
            specs_agnostic(url, Encryption::Sr25519, optional_token_override, None)
        })
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
        let meta = call_urls(&chain.rpc_endpoints, meta_fetch).map_err(anyhow::Error::msg)?;
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
