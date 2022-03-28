use definitions::{metadata::MetaValues};
use definitions::network_specs::NetworkSpecsToSend;


/// Struct to store MetaValues, genesis hash, and ChainSpecsToSend for network
pub struct MetaSpecs {
    pub meta_values: MetaValues,
    pub specs: NetworkSpecsToSend
}
