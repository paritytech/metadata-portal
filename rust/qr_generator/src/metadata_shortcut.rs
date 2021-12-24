use meta_reading::{decode_metadata::decode_version, fetch_metadata::{fetch_info, fetch_info_with_chainspecs}, interpret_chainspecs::interpret_properties};
use constants::{COLOR, SECONDARY_COLOR};
use definitions::{crypto::Encryption, metadata::MetaValues, network_specs::ChainSpecsToSend};
use std::convert::TryInto;
use std::fmt::format;
use db_handling::{helpers::unhex, error::NotHex};
use anyhow;
use meta_reading::fetch_metadata::FetchedInfoWithChainSpecs;
use crate::error::{Error, NotDecodeable};
use crate::export::{ChainSpecs, MetaSpecs};


/// Struct to store MetaValues and genesis hash for network
pub struct MetaShortCut {
    pub meta_values: MetaValues,
    pub genesis_hash: [u8; 32],
}

/// Function to process address as &str, fetch metadata and genesis hash for it,
/// and output MetaShortCut value in case of success
pub fn meta_shortcut (address: &str) -> anyhow::Result<MetaShortCut> {

    let new_info = match fetch_info(address) {
        Ok(a) => a,
        Err(e) => return Err(Error::FetchFailed{address: address.to_string(), error: e.to_string()}.show()),
    };
    let genesis_hash = get_genesis_hash(&new_info.genesis_hash)?;
    let meta_values = match decode_version(&new_info.meta) {
        Ok(a) => a,
        Err(e) => return Err(Error::NotDecodeable(NotDecodeable::FetchedMetadata{address: address.to_string(), error: e.to_string()}).show())
    };
    Ok(MetaShortCut{
        meta_values,
        genesis_hash,
    })
}


/// Struct to store MetaValues, genesis hash, and ChainSpecsToSend for network
pub struct MetaSpecsShortCut {
    pub meta_values: MetaValues,
    pub specs: ChainSpecsToSend,
    pub update: bool, // flag to indicate that the database has no exact entry created
}


/// Helper function to interpret freshly fetched genesis hash
fn get_genesis_hash (fetched_genesis_hash: &str) -> anyhow::Result<[u8; 32]> {
    let genesis_hash_vec = unhex(fetched_genesis_hash, NotHex::GenesisHash)?;
    let out: [u8; 32] = match genesis_hash_vec.try_into() {
        Ok(a) => a,
        Err(_) => return Err(Error::UnexpectedGenesisHashFormat.show())
    };
    Ok(out)
}


pub fn fetch_chain_info(address: &str) -> anyhow::Result<MetaSpecs>{
    let new_info: FetchedInfoWithChainSpecs = match fetch_info_with_chainspecs(address) {
        Ok(a) => a,
        Err(e) => return Err(Error::FetchFailed{address: address.to_string(), error: e.to_string()}.show()),
    };
    let genesis_hash = get_genesis_hash(&new_info.genesis_hash)?;
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
        color: COLOR.to_string(),
        decimals: new_properties.decimals,
        genesis_hash: format!("{:x?}", genesis_hash),
        logo: meta_values.name.to_string(),
        name: meta_values.name.to_string(),
        secondary_color: SECONDARY_COLOR.to_string(),
        unit: new_properties.unit.to_string(),
    };
    Ok(MetaSpecs{
        meta_values,
        specs
    })
}
