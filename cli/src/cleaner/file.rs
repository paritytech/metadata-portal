use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use anyhow::Context;

use crate::export::read_export_file;
use crate::qrs::{find_metadata_qrs, find_spec_qrs};
use crate::AppConfig;

pub(crate) fn files_to_remove(config: &AppConfig) -> anyhow::Result<Vec<PathBuf>> {
    log::debug!("files_to_remove()");

    let all_files: HashSet<PathBuf> = fs::read_dir(&config.qr_dir)
        .context(format!("{}", config.qr_dir.display()))?
        .map(|f| f.unwrap().path())
        .collect();

    let mut keep_files: HashSet<PathBuf> = HashSet::new();
    let metadata_qrs = find_metadata_qrs(&config.qr_dir)?;
    let specs_qrs = find_spec_qrs(&config.qr_dir)?;
    let chain_specs = read_export_file(config)?;

    for chain in &config.chains {
        let actual_ver = chain_specs
            .get(&chain.name)
            .context(format!("No data found for {}", chain.name))?
            .metadata_version;
        let metadata_to_keep = metadata_qrs
            .get(&chain.name)
            .map(|map| {
                map.iter()
                    .filter(|(&v, _)| v >= actual_ver)
                    .map(|(_, qr)| qr.to_path_buf())
                    .collect::<HashSet<_>>()
            })
            .context("Could not get metadata to keep")?;
        keep_files.extend(metadata_to_keep);
        if let Some(qr) = specs_qrs.get(&chain.name) {
            keep_files.insert(qr.to_path_buf());
        }
    }
    Ok(all_files.difference(&keep_files).cloned().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keep_future_versions() {
        let mut config = AppConfig::default();
        config.qr_dir = PathBuf::from("./src/cleaner/for_tests/test1/qrs");
        config.data_file = config.qr_dir.join("../data.json");

        let to_remove = files_to_remove(&config).unwrap();
        assert_eq!(to_remove.len(), 0);
    }

    #[test]
    fn remove_previous_versions() {
        let mut config = AppConfig::default();
        config.qr_dir = PathBuf::from("./src/cleaner/for_tests/test2/qrs");
        config.data_file = config.qr_dir.join("../data.json");

        let to_remove = files_to_remove(&config).unwrap();
        assert_eq!(to_remove.len(), 1);
        assert_eq!(to_remove[0], config.qr_dir.join("polkadot_metadata_9.apng"));
    }

    #[test]
    fn prefer_removing_unsigned() {
        let mut config = AppConfig::default();
        config.qr_dir = PathBuf::from("./src/cleaner/for_tests/test3/qrs");
        config.data_file = config.qr_dir.join("../data.json");

        let to_remove = files_to_remove(&config).unwrap();
        assert_eq!(to_remove.len(), 1);
        assert_eq!(
            to_remove[0],
            config.qr_dir.join("unsigned_polkadot_metadata_10.apng")
        );
    }

    #[test]
    fn not_in_config() {
        let mut config = AppConfig::default();
        config.qr_dir = PathBuf::from("./src/cleaner/for_tests/test4/qrs");
        config.data_file = config.qr_dir.join("../data.json");

        let to_remove = files_to_remove(&config).unwrap();
        assert_eq!(to_remove.len(), 1);
        assert_eq!(to_remove[0], config.qr_dir.join("kusama_metadata_9.apng"));
    }
}
