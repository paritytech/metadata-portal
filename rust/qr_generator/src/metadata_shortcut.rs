use meta_reading::{decode_metadata::decode_version, fetch_metadata::{fetch_info_with_chainspecs}, interpret_chainspecs::interpret_properties};
use anyhow;
use meta_reading::fetch_metadata::FetchedInfoWithChainSpecs;
use crate::error::{Error, NotDecodeable};
use crate::export::{ChainSpecs, MetaSpecs};


pub fn fetch_chain_info(address: &str) -> anyhow::Result<MetaSpecs>{
    let new_info: FetchedInfoWithChainSpecs = match fetch_info_with_chainspecs(address) {
        Ok(a) => a,
        Err(e) => return Err(Error::FetchFailed{address: address.to_string(), error: e.to_string()}.show()),
    };
    let meta_values = match decode_version(&new_info.meta) {
        Ok(a) => a,
        Err(e) => return Err(Error::NotDecodeable(NotDecodeable::FetchedMetadata{address: address.to_string(), error: e.to_string()}).show())
    };
    let new_properties = match interpret_properties(&new_info.properties) {
        Ok(a) => a,
        Err(e) => return Err(Error::BadNetworkProperties{address: address.to_string(), error: e.to_string()}.show()),
    };
    let specs = ChainSpecs {
        base58prefix: new_properties.base58prefix,
        decimals: new_properties.decimals,
        genesis_hash: new_info.genesis_hash,
        logo: meta_values.name.to_string(),
        name: meta_values.name.to_string(),
        unit: new_properties.unit.to_string(),
    };
    Ok(MetaSpecs{
        meta_values,
        specs
    })
}
