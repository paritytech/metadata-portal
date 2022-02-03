use definitions::{metadata::MetaValues};


/// Struct to store MetaValues, genesis hash, and ChainSpecsToSend for network
pub struct MetaSpecs {
    pub meta_values: MetaValues,
    pub specs: ChainSpecs
}

pub struct ChainSpecs {
    pub base58prefix: u16,
    pub decimals: u8,
    pub genesis_hash: String,
    pub logo: String,
    pub name: String,
    pub unit: String,
}
