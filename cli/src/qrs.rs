use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::utils::path::{ContentType, QrPath};
use crate::utils::types::{ChainName, SpecVersion};

type MetadataMap = HashMap<ChainName, BTreeMap<SpecVersion, QrPath>>;

/// QR dir content
pub(crate) fn qrs_in_dir(dir: impl AsRef<Path>) -> Result<Vec<QrPath>> {
    let mut files = vec![];
    for file in fs::read_dir(dir)? {
        let file = file?;
        if !file.file_type()?.is_file() {
            continue;
        }
        match QrPath::try_from(&file.path()) {
            Ok(qr_path) => files.push(qr_path),
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
    }
    Ok(files)
}

/// Maps chain to corresponding metadata QR files
pub(crate) fn find_metadata_qrs(dir: impl AsRef<Path>) -> Result<MetadataMap> {
    let mut metadata_qrs: HashMap<ChainName, BTreeMap<SpecVersion, QrPath>> = HashMap::new();
    for qr in qrs_in_dir(dir)? {
        if let ContentType::Metadata(version) = qr.file_name.content_type {
            metadata_qrs
                .entry(qr.file_name.chain.clone())
                .or_default()
                .entry(version)
                .and_modify(|e| {
                    if qr.file_name.is_signed {
                        *e = qr.clone();
                    }
                })
                .or_insert_with(|| qr.clone());
        }
    }
    Ok(metadata_qrs)
}

// Find all specs QR files in the given directory
pub(crate) fn find_spec_qrs(dir: impl AsRef<Path>) -> Result<HashMap<ChainName, QrPath>> {
    let mut specs_qrs = HashMap::new();
    for qr in qrs_in_dir(dir)? {
        if let ContentType::Specs = qr.file_name.content_type {
            specs_qrs
                .entry(qr.file_name.chain.clone())
                .and_modify(|e| {
                    if qr.file_name.is_signed {
                        *e = qr.clone();
                    }
                })
                .or_insert_with(|| qr.clone());
        }
    }
    Ok(specs_qrs)
}

// Helper function to extract metadata QR
pub(crate) fn extract_metadata_qr(
    metadata_qrs: &MetadataMap,
    chain: &ChainName,
    version: &SpecVersion,
) -> Result<QrPath> {
    let qr = metadata_qrs
        .get(chain)
        .and_then(|map| map.get(version))
        .context(format!(
            "No metadata found for {} version {}",
            chain, version
        ))?;
    Ok(qr.clone())
}

pub(crate) fn next_metadata_version(
    metadata_qrs: &MetadataMap,
    chain: &ChainName,
    active_version: SpecVersion,
) -> Result<Option<SpecVersion>> {
    let available_versions = metadata_qrs
        .get(chain)
        .map(|map| map.keys().copied().collect::<Vec<_>>())
        .context(format!("No metadata QRs for {}", chain))?;
    Ok(available_versions
        .iter()
        .find(|&v| *v > active_version)
        .copied())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn return_sorted_metadata() {
        let path = Path::new("./src/for_tests/sequential");
        let files = find_metadata_qrs(&path).unwrap();
        let mut result = files.get("kusama").unwrap().iter();
        let (first, _) = result.next().unwrap();
        assert_eq!(*first, 9);
        let (last, _) = result.next().unwrap();
        assert_eq!(*last, 10);
    }

    #[test]
    fn return_active_metadata() {
        let path = Path::new("./src/for_tests/sequential");
        let chain = &String::from("kusama");
        let map = find_metadata_qrs(&path).unwrap();
        let qr = extract_metadata_qr(&map, chain, &10);
        assert!(qr.is_ok());
        let qr = extract_metadata_qr(&map, chain, &11);
        assert!(qr.is_err());
    }

    #[test]
    fn test_next_metadata_version() {
        let path = Path::new("./src/for_tests/sequential");
        let chain = &String::from("kusama");
        let map = find_metadata_qrs(&path).unwrap();
        let v = next_metadata_version(&map, chain, 9).unwrap();
        assert!(v.is_some());
        assert_eq!(v.unwrap(), 10);

        let v = next_metadata_version(&map, chain, 10).unwrap();
        assert!(v.is_none());
    }

    #[test]
    fn prefer_signed_metadata() {
        let path = Path::new("./src/for_tests/signed_meta");
        let files = find_metadata_qrs(path).unwrap();
        let qr = files.get("polkadot").unwrap().get(&9001).unwrap();
        assert!(qr.file_name.is_signed);
    }

    #[test]
    fn return_latest_metadata_even_unsigned() {
        let path = Path::new("./src/for_tests/unsigned");
        let files = find_metadata_qrs(path).unwrap();

        let mut result = files.get("polkadot").unwrap().iter();
        let (first, qr) = result.next().unwrap();
        assert_eq!(*first, 9001);
        assert!(qr.file_name.is_signed);
        let (last, qr) = result.next().unwrap();
        assert_eq!(*last, 9002);
        assert!(!qr.file_name.is_signed);
    }
}
