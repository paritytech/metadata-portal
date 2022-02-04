use std::path::PathBuf;
use definitions::metadata::MetaValues;
use qrcode_rtx::make_pretty_qr;
use qr_lib::filename::QrFileName;


pub fn generate_metadata_qr(meta_values: &MetaValues, target_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    let crypto_type_code = "ff";
    let prelude = format!("53{}{}", crypto_type_code, "80");

    let file_name = QrFileName{
        chain: meta_values.name.clone(),
        kind: String::from("metadata"),
        version: meta_values.version,
        is_signed: false
    }.to_string();
    let path = target_dir.join(file_name);

    println!("generating QR for {}. It takes a while...", meta_values.name);
    let complete_message = [hex::decode(prelude).expect("known value"), meta_values.meta.clone()].concat();
    if let Err(e) = make_pretty_qr(&complete_message, &path.to_str().unwrap()) {
        anyhow::bail!("generating apng qr code: {}", e);
    }
    Ok(path)
}
