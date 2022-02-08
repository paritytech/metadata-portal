use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow;
use app_config::{AppConfig};
use qr_lib::filename::QrFileName;
use qr_lib::fs::latest_qr_per_chain;


pub fn full_run(config: AppConfig) -> anyhow::Result<()> {
    let chains: HashSet<String> = config.chains.into_iter().map(|chain| chain.name).collect();
    for path in files_to_remove(&config.qr_dir, chains)? {
        fs::remove_file(path)?;
    }
    Ok(())
}


fn files_to_remove(dir: impl AsRef<Path>, chains: HashSet<String>) -> anyhow::Result<Vec<PathBuf>> {
    let newest_qrs = latest_qr_per_chain(&dir)?;
    let keep: HashSet<QrFileName> = newest_qrs
        .into_iter()
        .filter(|(chain, _qr)| chains.contains(chain))
        .map(|(_chain, qr)| qr)
        .collect();
    let mut to_remove = vec![];
    for file in fs::read_dir(dir)? {
        let path = file?.path();
        if keep.contains(&QrFileName::try_from(&path)?) {
            continue
        }
        to_remove.push(path);
    }
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
        assert_eq!(to_remove[0], path.join("kusama_metadata_9"));
    }

    #[test]
    fn remove_if_not_in_config() {
        let path = Path::new("./for_tests/orphaned");
        let chains = HashSet::from(["kusama".to_string()]);
        let to_remove = files_to_remove(path, chains).unwrap();
        assert_eq!(to_remove.len(), 1);
        assert_eq!(to_remove[0], path.join("polkadot_metadata_10"));
    }

    #[test]
    fn prefer_removing_unsigned() {
        let path = Path::new("./for_tests/unsigned");
        let chains = HashSet::from(["kusama".to_string()]);
        let to_remove = files_to_remove(path, chains).unwrap();
        assert_eq!(to_remove.len(), 1);
        assert_eq!(to_remove[0], path.join("unsigned_kusama_metadata_10"));
    }
}
