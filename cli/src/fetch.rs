// use crate::collector::export::{ChainSpecs, MetaSpecs};
use anyhow::{anyhow, bail};
use definitions::error_active::IncomingMetadataSourceActiveStr;
use definitions::metadata::MetaValues;
use generate_message::interpret_specs::interpret_properties;
use generate_message::{fetch_metadata::fetch_info_with_network_specs, parser::TokenOverride};
use serde::{Deserialize, Serialize};

/// Struct to store MetaValues, genesis hash, and ChainSpecsToSend for network
pub(crate) struct MetaSpecs {
    pub(crate) meta_values: MetaValues,
    pub(crate) specs: ChainSpecs,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ChainSpecs {
    pub(crate) base58prefix: u16,
    pub(crate) decimals: u8,
    pub genesis_hash: String,
    pub logo: String,
    pub name: String,
    pub unit: String,
}

#[cfg(test)]
impl Default for ChainSpecs {
    fn default() -> Self {
        ChainSpecs {
            base58prefix: 0,
            decimals: 10,
            genesis_hash: "0x91b171bb150ce90c3".to_string(),
            logo: "logo".to_string(),
            name: "polkadot".to_string(),
            unit: "DOT".to_string(),
        }
    }
}

pub(crate) trait Fetcher {
    fn fetch_chain_info(
        &self,
        address: &str,
        token_unit: &Option<String>,
        token_decimals: &Option<u8>,
    ) -> anyhow::Result<MetaSpecs>;
}

pub(crate) struct RpcFetcher;

impl Fetcher for RpcFetcher {
    fn fetch_chain_info(
        &self,
        address: &str,
        token_unit: &Option<String>,
        token_decimals: &Option<u8>,
    ) -> anyhow::Result<MetaSpecs> {
        let new_info = match fetch_info_with_network_specs(address) {
            Ok(a) => a,
            Err(e) => bail!("failed to fetch chain info from {}: {}", address, e),
        };
        let meta_values = MetaValues::from_str_metadata(
            &new_info.meta,
            IncomingMetadataSourceActiveStr::Fetch {
                url: address.to_string(),
            },
        )
        .map_err(|e| anyhow!("{:?}", e))?;

        let optional_token_override =
            token_decimals
                .zip(token_unit.as_ref())
                .map(|(token_decimals, token_unit)| TokenOverride {
                    decimals: token_decimals,
                    unit: token_unit.to_string(),
                });

        let new_properties = match interpret_properties(
            &new_info.properties,
            meta_values.optional_base58prefix,
            optional_token_override,
        ) {
            Ok(a) => a,
            Err(e) => bail!("{:?}", e),
        };

        let specs = ChainSpecs {
            base58prefix: new_properties.base58prefix,
            decimals: new_properties.decimals,
            genesis_hash: new_info.genesis_hash,
            logo: meta_values.name.to_string(),
            name: meta_values.name.to_string(),
            unit: new_properties.unit,
        };
        Ok(MetaSpecs { meta_values, specs })
    }
}
