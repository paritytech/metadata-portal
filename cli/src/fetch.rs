use crate::config::Chain;
use crate::lib::string::{capitalize, hex_to_bytes};
use anyhow::{anyhow, bail};
use constants::{COLOR, SECONDARY_COLOR};
use definitions::crypto::Encryption;
use definitions::error_active::IncomingMetadataSourceActiveStr;
use definitions::metadata::MetaValues;
use definitions::network_specs::NetworkSpecsToSend;
use generate_message::interpret_specs::interpret_properties;
use generate_message::{fetch_metadata::fetch_info_with_network_specs, parser::TokenOverride};

/// Struct to store MetaValues, genesis hash, and ChainSpecsToSend for network
pub(crate) struct MetaSpecs {
    pub(crate) meta_values: MetaValues,
    pub(crate) specs: NetworkSpecsToSend,
}

pub(crate) trait Fetcher {
    fn fetch_chain_info(&self, chain: &Chain) -> anyhow::Result<MetaSpecs>;
}

pub(crate) struct RpcFetcher;

impl Fetcher for RpcFetcher {
    fn fetch_chain_info(&self, chain: &Chain) -> anyhow::Result<MetaSpecs> {
        let url = &chain.rpc_endpoint;
        let new_info = match fetch_info_with_network_specs(url) {
            Ok(a) => a,
            Err(e) => bail!("failed to fetch chain info from {}: {}", url, e),
        };
        let meta_values = MetaValues::from_str_metadata(
            &new_info.meta,
            IncomingMetadataSourceActiveStr::Fetch {
                url: url.to_string(),
            },
        )
        .map_err(|e| anyhow!("{:?}", e))?;

        let optional_token_override = chain.token_decimals.zip(chain.token_unit.as_ref()).map(
            |(token_decimals, token_unit)| TokenOverride {
                decimals: token_decimals,
                unit: token_unit.to_string(),
            },
        );

        let new_properties = match interpret_properties(
            &new_info.properties,
            meta_values.optional_base58prefix,
            optional_token_override,
        ) {
            Ok(a) => a,
            Err(e) => bail!("{:?}", e),
        };
        let encryption = Encryption::Sr25519;
        let genesis_hash: [u8; 32] = hex_to_bytes(&new_info.genesis_hash)?.try_into().unwrap();

        let specs = NetworkSpecsToSend {
            base58prefix: new_properties.base58prefix,
            color: COLOR.to_string(),
            decimals: new_properties.decimals,
            encryption,
            genesis_hash,
            logo: meta_values.name.to_string(),
            name: meta_values.name.to_string(),
            path_id: format!("//{}", meta_values.name),
            secondary_color: SECONDARY_COLOR.to_string(),
            title: capitalize(&meta_values.name),
            unit: new_properties.unit,
        };
        Ok(MetaSpecs { meta_values, specs })
    }
}
