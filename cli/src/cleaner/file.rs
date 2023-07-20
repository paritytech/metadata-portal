use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use anyhow::Context;

use crate::file::files_to_keep;
use crate::AppConfig;

pub(crate) fn files_to_remove(config: &AppConfig) -> anyhow::Result<Vec<PathBuf>> {
    let all_files: HashSet<PathBuf> = fs::read_dir(&config.qr_dir)
        .context(format!("{}", config.qr_dir.display()))?
        .map(|f| f.unwrap().path())
        .collect();

    let keep_files = files_to_keep(config)?
        .iter()
        .map(|qr| qr.to_path_buf())
        .collect::<HashSet<_>>();
    Ok(all_files.difference(&keep_files).cloned().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Chain;

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

    #[test]
    fn works_with_parachain_qrs() {
        let mut config = AppConfig::default();
        config.chains = vec![
            Chain::default(),
            Chain {
                name: "statemint".to_string(),
                relay_chain: Some("polkadot".to_string()),
                ..Chain::default()
            },
        ];
        config.qr_dir = PathBuf::from("./src/cleaner/for_tests/test5/qrs");
        config.data_file = config.qr_dir.join("../data.json");

        let to_remove = files_to_remove(&config).unwrap();
        assert_eq!(to_remove.len(), 0);
    }
}
