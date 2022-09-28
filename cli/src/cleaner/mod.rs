mod file;

use std::fs;

use crate::cleaner::file::files_to_remove;
use crate::config::AppConfig;

pub(crate) fn clean(config: AppConfig) -> anyhow::Result<()> {
    let files = files_to_remove(&config)?;
    if files.is_empty() {
        println!("✔ Nothing to delete");
        return Ok(());
    }
    for path in files {
        fs::remove_file(&path)?;
        println!("🗑 {} was deleted", path.display());
    }
    Ok(())
}
