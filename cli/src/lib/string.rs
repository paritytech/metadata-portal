pub(crate) fn hex_to_bytes(hex_entry: &str) -> anyhow::Result<Vec<u8>> {
    let hex_entry = hex_entry.strip_prefix("0x").unwrap_or(hex_entry);
    Ok(hex::decode(hex_entry)?)
}
