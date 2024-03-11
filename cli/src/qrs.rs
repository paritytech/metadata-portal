use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::common::path::{ContentType, QrPath};
use crate::common::types::{ChainPortalId, MetaVersion};

type MetadataMap = HashMap<ChainPortalId, BTreeMap<MetaVersion, QrPath>>;

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
                eprintln!("{e}");
                continue;
            }
        }
    }
    Ok(files)
}

/// Maps chain to corresponding metadata QR files
pub(crate) fn metadata_files(dir: impl AsRef<Path>) -> Result<MetadataMap> {
    let mut metadata_qrs: MetadataMap = HashMap::new();
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
pub(crate) fn spec_files(dir: impl AsRef<Path>) -> Result<HashMap<ChainPortalId, QrPath>> {
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
pub(crate) fn collect_metadata_qrs(
    all_metadata: &MetadataMap,
    chain_portal_id: &ChainPortalId,
    live_version: &MetaVersion,
) -> Result<Vec<QrPath>> {
    let mut metadata_qrs = vec![];
    for (version, qr) in all_metadata
        .get(chain_portal_id.as_str())
        .with_context(|| format!("No metadata qr found for {}", chain_portal_id))?
        .iter()
    {
        if version <= live_version {
            // only keep the latest metadata
            metadata_qrs = vec![qr.clone()];
        } else {
            metadata_qrs.push(qr.clone());
        }
    }
    Ok(metadata_qrs)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn return_sorted_metadata() {
        let path = Path::new("./src/for_tests/sequential");
        let files = metadata_files(path).unwrap();
        let mut result = files.get("kusama").unwrap().iter();
        let (first, _) = result.next().unwrap();
        assert_eq!(*first, 9);
        let (last, _) = result.next().unwrap();
        assert_eq!(*last, 10);
    }

    #[test]
    fn test_collect_metadata() {
        let path = Path::new("./src/for_tests/sequential");
        let chain = &String::from("kusama");
        let map = metadata_files(path).unwrap();
        let qrs = collect_metadata_qrs(&map, chain, &9).unwrap();
        assert_eq!(qrs.len(), 2);
        let qrs = collect_metadata_qrs(&map, chain, &10).unwrap();
        assert_eq!(qrs.len(), 1);
        let qrs = collect_metadata_qrs(&map, chain, &11).unwrap();
        assert_eq!(qrs.len(), 1);
    }

    #[test]
    fn prefer_signed_metadata() {
        let path = Path::new("./src/for_tests/signed_meta");
        let files = metadata_files(path).unwrap();
        let qr = files.get("polkadot").unwrap().get(&9001).unwrap();
        assert!(qr.file_name.is_signed);
    }

    #[test]
    fn return_latest_metadata_even_unsigned() {
        let path = Path::new("./src/for_tests/unsigned");
        let files = metadata_files(path).unwrap();

        let mut result = files.get("polkadot").unwrap().iter();
        let (first, qr) = result.next().unwrap();
        assert_eq!(*first, 9001);
        assert!(qr.file_name.is_signed);
        let (last, qr) = result.next().unwrap();
        assert_eq!(*last, 9002);
        assert!(!qr.file_name.is_signed);
    }
}
