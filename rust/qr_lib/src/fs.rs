use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;
use crate::filename::QrFileName;

type ChainName = String;


pub fn read_qr_dir(dir: &PathBuf) -> anyhow::Result<HashMap<ChainName, QrFileName>>{
    let mut newest: HashMap<ChainName, QrFileName>= HashMap::new();
    for file in fs::read_dir(dir)? {
        let path = file?.path();
        let qr_file = QrFileName::try_from(path.clone())?;

        match newest.get(&qr_file.chain) {
            Some(latest) if latest.version >= qr_file.version => (),
            Some(latest) if latest.version == qr_file.version && latest.is_signed => (),
            _ => {
                newest.insert(String::from(&qr_file.chain), qr_file.clone());
            }
        }
    }
    Ok(newest)
}
