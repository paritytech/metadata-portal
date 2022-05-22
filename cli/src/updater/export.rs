use definitions::metadata::MetaValues;
use definitions::network_specs::NetworkSpecsToSend;

/// Struct to store MetaValues, genesis hash, and ChainSpecsToSend for network
pub(crate) struct MetaSpecs {
    pub(crate) meta_values: MetaValues,
    pub(crate) specs: NetworkSpecsToSend,
}
