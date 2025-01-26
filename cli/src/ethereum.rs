const ETHEREUM_CHAINS: [&str; 12] = [
    "jaz",
    "moonbase",
    "moonbeam",
    "moonriver",
    "moonshadow",
    "ipblockchain",
    "ipblockchain-testnet",
    "alt-producer",
    "flash-layer",
    "armonia-eva",
    "armonia-wall-e",
    "root",
];

pub(crate) fn is_ethereum(chain_name: &str) -> bool {
    ETHEREUM_CHAINS.contains(&chain_name)
}
