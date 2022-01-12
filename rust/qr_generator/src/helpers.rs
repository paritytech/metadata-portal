use sled::{IVec, Batch};
use anyhow;
use constants::{ADDRESS_BOOK, HOT_DB_NAME, SPECSTREEPREP};
use definitions::{crypto::Encryption, keyring::{AddressBookKey, NetworkSpecsKey, MetaKey}, metadata::{AddressBookEntry, MetaValues, VersionDecoded}, network_specs::ChainSpecsToSend};
use meta_reading::decode_metadata::get_meta_const;
use parity_scale_codec::{Decode, Encode};


use crate::error::{Error, NotDecodeable, NotFound};

/// Function to decode SCALE encoded network specs into ChainSpecsToSend,
/// and check for genesis hash mismatch
pub fn decode_chain_specs_to_send(chain_specs_to_send_encoded: IVec, network_specs_key: &NetworkSpecsKey) -> anyhow::Result<ChainSpecsToSend> {
    match <ChainSpecsToSend>::decode(&mut &chain_specs_to_send_encoded[..]) {
        Ok(a) => {
            if network_specs_key != &NetworkSpecsKey::from_parts(&a.genesis_hash.to_vec(), &a.encryption) {return Err(Error::NetworkSpecsKeyMismatch(a.name).show())}
            Ok(a)
        },
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::ChainSpecsToSend).show()),
    }
}

/// Function to decode and check for integrity an entry from metadata database
pub fn decode_and_check_meta_entry ((meta_key_vec, meta): (IVec, IVec)) -> anyhow::Result<MetaValues> {
// decode what is in the key
    let (name, version) = match MetaKey::from_vec(&meta_key_vec.to_vec()).name_version() {
        Ok(a) => a,
        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::DatabaseVersionedName).show()),
    };
// check the database for corruption
    let version_vector = match get_meta_const(&meta.to_vec()) {
        Ok(a) => a,
        Err(e) => return Err(Error::DatabaseMetadata{name, version, error: e.to_string()}.show()),
    };
    let version_decoded = match VersionDecoded::decode(&mut &version_vector[..]) {
        Ok(a) => a,
        Err(e) => return Err(Error::DatabaseMetadata{name, version, error: e.to_string()}.show()),
    };
    if (version_decoded.specname != name)||(version_decoded.spec_version != version) {return Err(Error::DatabaseMetadataMismatch{name1: name, version1: version, name2: version_decoded.specname, version2: version_decoded.spec_version}.show())}
// output
    Ok(MetaValues {
        name,
        version,
        meta: meta.to_vec(),
    })
}

/// Function to process error depending on pass_errors flag
pub fn error_occured (e: anyhow::Error, pass_errors: bool) -> anyhow::Result<()> {
    if pass_errors {Ok(println!("Error encountered. {} Skipping it.", e))}
    else {return Err(e)}
}

/// Enum to indicate what need to be printed in `load_metadata` and `add_network` messages
pub enum Write {
    All, // -t key or no set key
    OnlyNew, // -k key
    None, // -p key
}

/// Struct to store indices (id found) for correct encryption and for default entry
struct Indices {
    index_correct_encryption: Option<usize>,
    index_default: Option<usize>,
}

/// Function to search through a vector of AddressBookEntry (for use with sets having the same address)
/// for entry with given encryption and for default entry;
/// Checks that there is only one default entry and only one entry with given encryption for this address
fn get_indices (entries: &Vec<AddressBookEntry>, encryption: Encryption) -> anyhow::Result<Indices> {
    let mut index_correct_encryption = None;
    let mut index_default = None;
    for (i, x) in entries.iter().enumerate() {
        if x.encryption == encryption {
            match index_correct_encryption {
                Some(_) => return Err(Error::TwoEntriesAddressEncryption{address: x.address.to_string(), encryption}.show()),
                None => {index_correct_encryption = Some(i)}
            }
        }
        if x.def {
            match index_default {
                Some(_) => return Err(Error::TwoDefaultsAddress(x.address.to_string()).show()),
                None => {index_default = Some(i)}
            }
        }
    }
    Ok(Indices{
        index_correct_encryption,
        index_default,
    })
}
