use crate::collector::export::{ChainSpecs, MetaSpecs};
use anyhow::{anyhow, bail};
use definitions::error::IncomingMetadataSourceActiveStr;
use definitions::metadata::MetaValues;
use generate_message::fetch_metadata::fetch_info_with_network_specs;
use generate_message::interpret_specs::interpret_properties;
use generate_message::parser::TokenOverride;

pub fn fetch_chain_info(
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
