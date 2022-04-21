use crate::path::{ContentType, QrPath};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::path::Path;

type ChainName = String;
type Version = u32;

pub fn metadata_qr_in_dir(
    dir: impl AsRef<Path>,
) -> anyhow::Result<HashMap<ChainName, (QrPath, Version)>> {
    let mut latest_qrs: HashMap<ChainName, (QrPath, Version)> = HashMap::new();

    for qr_path in raw_read_qr_dir(dir)? {
        let current = &qr_path.file_name;
        if let ContentType::Metadata(current_version) = current.content_type {
            match latest_qrs.get(&current.chain) {
                Some((_, latest_version)) if latest_version > &current_version => (),
                Some((qr, v)) if v == &current_version && qr.file_name.is_signed => (),
                _ => {
                    latest_qrs.insert(
                        String::from(&current.chain),
                        (qr_path.clone(), current_version),
                    );
                }
            };
        }
    }
    Ok(latest_qrs)
}

pub fn specs_qr_in_dir(dir: impl AsRef<Path>) -> anyhow::Result<HashMap<ChainName, QrPath>> {
    let mut latest_qrs: HashMap<ChainName, QrPath> = HashMap::new();

    for qr_path in raw_read_qr_dir(dir)? {
        let current = &qr_path.file_name;
        if let ContentType::Specs = current.content_type {
            match latest_qrs.get(&current.chain) {
                Some(qr) if qr.file_name.is_signed => (),
                _ => {
                    latest_qrs.insert(String::from(&current.chain), qr_path.clone());
                }
            };
        }
    }
    Ok(latest_qrs)
}

// Get all latest QRs.
pub fn all_qrs_in_dir(dir: impl AsRef<Path>) -> anyhow::Result<Vec<QrPath>> {
    let metadata_qrs: Vec<QrPath> = metadata_qr_in_dir(&dir)?
        .values()
        .map(|(qr, _)| qr.to_owned())
        .collect();
    let specs_qrs: Vec<QrPath> = specs_qr_in_dir(&dir)?
        .values()
        .map(|qr| qr.to_owned())
        .collect();
    Ok([metadata_qrs, specs_qrs].concat())
}

// Read QR dir without filtration. There might be outdated QRs.
pub fn raw_read_qr_dir(dir: impl AsRef<Path>) -> anyhow::Result<Vec<QrPath>> {
    let mut files = vec![];
    for file in fs::read_dir(dir)? {
        let path = file?.path();
        files.push(QrPath::try_from(&path)?)
    }
    Ok(files)
}

pub fn hex_to_bytes(hex_entry: &str) -> anyhow::Result<Vec<u8>> {
    let hex_entry = hex_entry.strip_prefix("0x").unwrap_or(hex_entry);
    Ok(hex::decode(hex_entry)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path::ContentType;
    use std::path::Path;

    #[test]
    fn return_latest_metadata() {
        let path = Path::new("./for_tests/happy");
        let files = metadata_qr_in_dir(&path).unwrap();
        let (result, _) = files.get("polkadot").unwrap();
        assert_eq!(result.file_name.content_type, ContentType::Metadata(9002));
    }

    #[test]
    fn return_specs() {
        let path = Path::new("./for_tests/happy");
        let files = specs_qr_in_dir(&path).unwrap();
        let result = files.get("polkadot").unwrap();
        assert_eq!(result.file_name.content_type, ContentType::Specs);
    }

    #[test]
    fn prefer_signed_metadata() {
        let path = Path::new("./for_tests/signed");
        let files = metadata_qr_in_dir(&path.to_path_buf()).unwrap();
        let (result, _) = files.get("polkadot").unwrap();
        assert!(result.file_name.is_signed);
    }

    #[test]
    fn prefer_signed_specs() {
        let path = Path::new("./for_tests/signed");
        let files = specs_qr_in_dir(&path.to_path_buf()).unwrap();
        let result = files.get("polkadot").unwrap();
        assert!(result.file_name.is_signed);
    }

    #[test]
    fn return_latest_metadata_even_unsigned() {
        let path = Path::new("./for_tests/unsigned");
        let files = metadata_qr_in_dir(&path.to_path_buf()).unwrap();
        let (result, _) = files.get("polkadot").unwrap();
        assert!(!result.file_name.is_signed);
        assert_eq!(result.file_name.content_type, ContentType::Metadata(9002));
    }

    #[test]
    fn return_specsa_even_unsigned() {
        let path = Path::new("./for_tests/unsigned");
        let files = specs_qr_in_dir(&path.to_path_buf()).unwrap();
        let result = files.get("polkadot").unwrap();
        assert!(!result.file_name.is_signed);
        assert_eq!(result.file_name.content_type, ContentType::Specs);
    }

    #[test]
    fn test_all_qrs_in_dir() {
        let path = Path::new("./for_tests/happy");
        let files = all_qrs_in_dir(&path.to_path_buf()).unwrap();
        assert_eq!(files.len(), 2);
    }
}
