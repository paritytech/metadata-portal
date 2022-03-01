use std::collections::HashSet;
use std::fs;
use std::path::{Path};

use anyhow;
use app_config::{AppConfig};
use qr_lib::path::{QrPath};
use qr_lib::read::{latest_qr_per_chain, read_qr_dir};


pub fn full_run(config: AppConfig) -> anyhow::Result<()> {
    let chains: HashSet<String> = config.chains.into_iter().map(|chain| chain.name).collect();
    let files = files_to_remove(&config.qr_dir, chains)?;
    if files.is_empty() {
        println!("✔ Nothing to delete");
        return Ok(())
    }
    for path in files {
        fs::remove_file(path.to_path_buf())?;
        println!("🗑 {} was deleted", path);
    }
    Ok(())
}


fn files_to_remove(dir: impl AsRef<Path>, chains: HashSet<String>) -> anyhow::Result<Vec<QrPath>> {
    let newest_qrs = latest_qr_per_chain(&dir)?;
    let keep: HashSet<QrPath> = newest_qrs
        .into_iter()
        .filter(|(chain, _)| chains.contains(chain))
        .map(|(_chain, qr)| qr)
        .collect();

    let to_remove = read_qr_dir(&dir)?
        .into_iter()
        .filter(|qr_path| !keep.contains(qr_path))
        .collect();
     Ok(to_remove)
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn remove_previous_version() {
        let path = Path::new("./for_tests/previous");
        let chains = HashSet::from(["kusama".to_string()]);
        let to_remove = files_to_remove(path, chains).unwrap();
        assert_eq!(to_remove.len(), 1);
        assert_eq!(to_remove[0].to_path_buf(), path.join("kusama_metadata_9"));
    }

    #[test]
    fn remove_if_not_in_config() {
        let path = Path::new("./for_tests/orphaned");
        let chains = HashSet::from(["kusama".to_string()]);
        let to_remove = files_to_remove(path, chains).unwrap();
        assert_eq!(to_remove.len(), 1);
        assert_eq!(to_remove[0].to_path_buf(), path.join("polkadot_metadata_10"));
    }

    #[test]
    fn prefer_removing_unsigned() {
        let path = Path::new("./for_tests/unsigned");
        let chains = HashSet::from(["kusama".to_string()]);
        let to_remove = files_to_remove(path, chains).unwrap();
        assert_eq!(to_remove.len(), 1);
        assert_eq!(to_remove[0].to_path_buf(), path.join("unsigned_kusama_metadata_10"));
    }
}
