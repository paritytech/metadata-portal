use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::Serialize;

pub(crate) fn save_to_file<T>(specs: &T, path: impl AsRef<Path>) -> anyhow::Result<()>
where
    T: ?Sized + Serialize,
{
    let serialized = serde_json::to_string_pretty(specs).unwrap();
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;
    file.write_all("\n".as_bytes())?;
    Ok(())
}
