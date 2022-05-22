use crate::lib::string::capitalize;
use crate::lib::string::hex_to_bytes;
use crate::updater::export::MetaSpecs;
use anyhow::{anyhow, bail};
use constants::{COLOR, SECONDARY_COLOR};
use definitions::crypto::Encryption;
use definitions::error_active::IncomingMetadataSourceActiveStr;
use definitions::metadata::MetaValues;
use definitions::network_specs::NetworkSpecsToSend;
use generate_message::fetch_metadata::fetch_info_with_network_specs;
use generate_message::interpret_specs::interpret_properties;
use std::convert::TryInto;

pub(crate) fn fetch_chain_info(address: &str) -> anyhow::Result<MetaSpecs> {
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

    let new_properties = match interpret_properties(
        &new_info.properties,
        meta_values.optional_base58prefix,
        None,
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
