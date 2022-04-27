use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::config::AppConfig;
use crate::lib::path::QrPath;
use crate::lib::read::{all_qrs_in_dir, raw_read_qr_dir};

pub fn clean(config: AppConfig) -> anyhow::Result<()> {
    let chains: HashSet<String> = config.chains.into_iter().map(|chain| chain.name).collect();
    let files = files_to_remove(&config.qr_dir, chains)?;
    if files.is_empty() {
        println!("✔ Nothing to delete");
        return Ok(());
    }
    for path in files {
        fs::remove_file(path.to_path_buf())?;
        println!("🗑 {} was deleted", path);
    }
    Ok(())
}

fn files_to_remove(dir: impl AsRef<Path>, chains: HashSet<String>) -> anyhow::Result<Vec<QrPath>> {
    let newest_qrs = all_qrs_in_dir(&dir)?;
    let keep: HashSet<QrPath> = newest_qrs
        .into_iter()
        .filter(|qr| chains.contains(&qr.file_name.chain))
        .collect();

    let to_remove = raw_read_qr_dir(&dir)?
        .into_iter()
        .filter(|qr_path| !keep.contains(qr_path))
        .collect();
    Ok(to_remove)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn remove_previous_version() {
        let path = Path::new("./for_tests/sequential");
        let chains = HashSet::from(["kusama".to_string()]);
        let to_remove = files_to_remove(path, chains).unwrap();
        assert_eq!(to_remove.len(), 1);
        assert_eq!(
            to_remove[0].to_path_buf(),
            path.join("kusama_metadata_9.apng")
        );
    }

    #[test]
    fn remove_if_not_in_config() {
        let path = Path::new("./for_tests/different_chains");
        let chains = HashSet::from(["kusama".to_string()]);
        let to_remove = files_to_remove(path, chains).unwrap();
        assert_eq!(to_remove.len(), 1);
        assert_eq!(
            to_remove[0].to_path_buf(),
            path.join("polkadot_metadata_10")
        );
    }

    #[test]
    fn prefer_removing_unsigned() {
        let path = Path::new("./for_tests/signed_meta");
        let chains = HashSet::from(["polkadot".to_string()]);
        let to_remove = files_to_remove(path, chains).unwrap();
        assert_eq!(to_remove.len(), 1);
        assert_eq!(
            to_remove[0].to_path_buf(),
            path.join("unsigned_polkadot_metadata_9001")
        );
    }
}
