/// Capitalizes the first character in s.
pub(crate) fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub(crate) fn hex_to_bytes(hex_entry: &str) -> anyhow::Result<Vec<u8>> {
    let hex_entry = hex_entry.strip_prefix("0x").unwrap_or(hex_entry);
    Ok(hex::decode(hex_entry)?)
}
