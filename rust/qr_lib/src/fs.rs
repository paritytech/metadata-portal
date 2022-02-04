use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::path::{Path};
use crate::filename::QrFileName;

type ChainName = String;


pub fn read_qr_dir(dir: impl AsRef<Path>) -> anyhow::Result<HashMap<ChainName, QrFileName>>{
    let mut newest: HashMap<ChainName, QrFileName>= HashMap::new();
    for file in fs::read_dir(dir)? {
        let path = file?.path();
        let qr_file = QrFileName::try_from(path.clone())?;

        match newest.get(&qr_file.chain) {
            Some(latest) if latest.version > qr_file.version => (),
            Some(latest) if latest.version == qr_file.version && latest.is_signed => (),
            _ => {
                newest.insert(String::from(&qr_file.chain), qr_file.clone());
            }
        }
    }
    Ok(newest)
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn return_latest() {
        let path = Path::new("./for_tests/happy");
        let files = read_qr_dir(&path.to_path_buf()).unwrap();
        let result = files.get("polkadot").unwrap();
        assert_eq!(result.version, 9002);
    }

    #[test]
    fn prefer_signed() {
        let path = Path::new("./for_tests/signed");
        let files = read_qr_dir(&path.to_path_buf()).unwrap();
        let result = files.get("polkadot").unwrap();
        assert!(result.is_signed);
    }

    #[test]
    fn return_latest_even_unsigned() {
        let path = Path::new("./for_tests/unsigned");
        let files = read_qr_dir(&path.to_path_buf()).unwrap();
        let result = files.get("polkadot").unwrap();
        assert!(!result.is_signed);
        assert_eq!(result.version, 9002);
    }
}
