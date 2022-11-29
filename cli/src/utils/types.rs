use definitions::crypto::Encryption;

use crate::config::Chain;

pub(crate) type ChainName = String;
pub(crate) type SpecVersion = u32;

pub(crate) fn get_crypto(chain: &Chain) -> Encryption {
    match &chain.encryption {
        Some(encryption) => {
            if encryption == "ethereum" {
                Encryption::Ethereum
            } else {
                Encryption::Sr25519
            }
        }
        _ => Encryption::Sr25519,
    }
}
