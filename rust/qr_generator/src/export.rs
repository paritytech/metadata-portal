use serde::{Serialize, Deserialize};

use definitions::{crypto::Encryption, metadata::MetaValues};

/// Struct to store MetaValues, genesis hash, and ChainSpecsToSend for network
pub struct MetaSpecs {
    pub meta_values: MetaValues,
    pub specs: ChainSpecs
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChainSpecs {
    pub base58prefix: u16,
    pub color: String,
    pub decimals: u8,
    pub genesis_hash: String,
    pub logo: String,
    pub name: String,
    pub secondary_color: String,
    pub unit: String,
}