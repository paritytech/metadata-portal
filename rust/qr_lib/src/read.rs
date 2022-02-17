use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::path::{Path};
use crate::path::{QrPath};

type ChainName = String;


pub fn latest_qr_per_chain(dir: impl AsRef<Path>) -> anyhow::Result<HashMap<ChainName, QrPath>>{
    let mut newest: HashMap<ChainName, QrPath> = HashMap::new();

    for qr_path in read_qr_dir(dir)? {
        let current = &qr_path.file_name;
        match newest.get(&current.chain) {
            Some(latest) if latest.file_name.version > current.version => (),
            Some(latest) if latest.file_name.version == current.version && latest.file_name.is_signed => (),
            _ => {
                newest.insert(String::from(&current.chain), qr_path.clone());
            }
        }
    }
    Ok(newest)
}


pub fn read_qr_dir(dir: impl AsRef<Path>) -> anyhow::Result<Vec<QrPath>>{
    let mut files = vec![];
    for file in fs::read_dir(dir)? {
        let path = file?.path();
        files.push(QrPath::try_from(&path)?)
    }
    Ok(files)
}

pub fn hex_to_bytes(hex_entry: &str) -> anyhow::Result<Vec<u8>> {
    let hex_entry = {
        if hex_entry.starts_with("0x") {&hex_entry[2..]}
        else {hex_entry}
    };
    Ok(hex::decode(hex_entry)?)
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn return_latest() {
        let path = Path::new("./for_tests/happy");
        let files = latest_qr_per_chain(&path).unwrap();
        let result = files.get("polkadot").unwrap();
        assert_eq!(result.file_name.version, 9002);
    }

    #[test]
    fn prefer_signed() {
        let path = Path::new("./for_tests/signed");
        let files = latest_qr_per_chain(&path.to_path_buf()).unwrap();
        let result = files.get("polkadot").unwrap();
        assert!(result.file_name.is_signed);
    }

    #[test]
    fn return_latest_even_unsigned() {
        let path = Path::new("./for_tests/unsigned");
        let files = latest_qr_per_chain(&path.to_path_buf()).unwrap();
        let result = files.get("polkadot").unwrap();
        assert!(!result.file_name.is_signed);
        assert_eq!(result.file_name.version, 9002);
    }
}
