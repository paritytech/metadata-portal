use std::path::{Path, PathBuf};

use anyhow::bail;
use definitions::crypto::SufficientCrypto;
use definitions::metadata::MetaValues;
use definitions::network_specs::NetworkSpecsToSend;
use definitions::qr_transfers::{ContentAddSpecs, ContentLoadMeta};
use generate_message::full_run;
use generate_message::parser::{Command as SignerCommand, Crypto, Goal, Make, Msg};
use log::info;
use sp_core::{sr25519, Pair, H256};

use crate::lib::path::{ContentType, QrFileName};
use crate::lib::types::ChainName;

pub(crate) fn generate_metadata_qr(
    meta_values: &MetaValues,
    genesis_hash: &H256,
    target_dir: &Path,
    sign: bool,
    signing_key: String,
) -> anyhow::Result<PathBuf> {
    let content = ContentLoadMeta::generate(&meta_values.meta, genesis_hash);

    let file_name = QrFileName::new(
        &meta_values.name.to_lowercase(),
        ContentType::Metadata(meta_values.version),
        sign,
    )
    .to_string();
    let path = target_dir.join(&file_name);
    let make;
    if sign {
        let sr25519_pair = match sr25519::Pair::from_string(signing_key.as_str(), None) {
            Ok(x) => x,
            Err(_e) => {
                bail!("❌ Key error. Generate metadata with `make updater` and sign manually")
            }
        };
        let signature = sr25519_pair.sign(content.to_sign().as_slice());
        make = Make {
            goal: Goal::Qr,
            crypto: Crypto::Sufficient(SufficientCrypto::Sr25519 {
                public: sr25519_pair.public(),
                signature,
            }),
            msg: Msg::LoadMetadata(content.to_sign()),
            name: Some(path.to_str().unwrap().to_owned()),
        };
    } else {
        make = Make {
            goal: Goal::Qr,
            crypto: Crypto::None,
            msg: Msg::LoadMetadata(content.to_sign()),
            name: Some(path.to_str().unwrap().to_owned()),
        };
    }
    info!("⚙️  Generating {}...", file_name);
    full_run(SignerCommand::Make(make)).map_err(anyhow::Error::msg)?;
    Ok(path)
}

pub(crate) fn generate_spec_qr(
    name: &ChainName,
    specs: &NetworkSpecsToSend,
    target_dir: &Path,
) -> anyhow::Result<PathBuf> {
    let file_name = QrFileName::new(&name.to_lowercase(), ContentType::Specs, false).to_string();
    let path = target_dir.join(&file_name);
    let content = ContentAddSpecs::generate(specs);

    let make = Make {
        goal: Goal::Qr,
        crypto: Crypto::None,
        msg: Msg::AddSpecs(content.to_sign()),
        name: Some(path.to_str().unwrap().to_owned()),
    };
    info!("⚙️  Generating {}...", file_name);
    full_run(SignerCommand::Make(make)).map_err(anyhow::Error::msg)?;
    Ok(path)
}
