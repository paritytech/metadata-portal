mod qr;

use anyhow::Result;
use log::info;

use crate::verifier::qr::validate_signed_qrs;
use crate::AppConfig;

pub(crate) fn verify(config: AppConfig) -> Result<()> {
    validate_signed_qrs(&config.qr_dir, &config.verifier.public_key)?;
    info!("✅ Done");
    Ok(())
}
