use std::collections::HashMap;
use std::convert::TryFrom;
use std::{fs};
use std::path::PathBuf;
use definitions::metadata::MetaValues;
use qrcode_rtx::make_pretty_qr;
use file_names::QrFileName;
use crate::{QrCode, ReactAssetPath};


pub fn read_qr_dir(result: &mut HashMap<String, QrCode>, dir: &PathBuf, public_dir: &PathBuf, is_verified: bool) -> anyhow::Result<()>{
    for file in fs::read_dir(dir)? {
        let path = file?.path();
        let qr_file = QrFileName::try_from(path.clone())?;

        match result.get(&qr_file.chain).map(|qr| qr.version) {
            Some(newest_version) if newest_version >= qr_file.version => (),
            _ => {
                result.insert(String::from(qr_file.chain), QrCode {
                    path: ReactAssetPath::from_fs_path(path, public_dir)?,
                    is_verified,
                    version: qr_file.version,
                });
            }
        }
    }
    Ok(())
}

pub fn generate_metadata_qr(meta_values: &MetaValues, target_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    let crypto_type_code = "ff";
    let prelude = format!("53{}{}", crypto_type_code, "80");

    let file_name = QrFileName{
        chain: meta_values.name.clone(),
        kind: String::from("metadata"),
        version: meta_values.version
    }.to_string();
    let path = target_dir.join(file_name);

    println!("generating QR for {}. It takes a while...", meta_values.name);
    let complete_message = [hex::decode(prelude).expect("known value"), meta_values.meta.clone()].concat();
    if let Err(e) = make_pretty_qr(&complete_message, &path.to_str().unwrap()) {
        anyhow::bail!("generating apng qr code: {}", e);
    }
    Ok(path)
}
