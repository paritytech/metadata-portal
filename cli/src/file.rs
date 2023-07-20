use std::collections::HashSet;

use anyhow::Context;

use crate::common::path::QrPath;
use crate::export::read_export_file;
use crate::qrs::{metadata_files, spec_files};
use crate::AppConfig;

pub(crate) fn files_to_keep(config: &AppConfig) -> anyhow::Result<HashSet<QrPath>> {
    let mut keep_files: HashSet<QrPath> = HashSet::new();
    let all_metadata = metadata_files(&config.qr_dir)?;
    let all_specs = spec_files(&config.qr_dir)?;
    let chain_specs = read_export_file(config)?;

    for chain in &config.chains {
        let latest_version = match &chain_specs
            .get(&chain.portal_id())
            .context(format!("No data found for {}", chain.portal_id()))?
            .metadata_qr
        {
            Some(qr) => qr.version,
            None => continue,
        };
        let metadata_to_keep = all_metadata
            .get(&chain.portal_id())
            .map(|map| {
                map.iter()
                    .filter(|(&v, _)| v >= latest_version)
                    .map(|(_, qr)| qr.to_owned())
                    .collect::<HashSet<_>>()
            })
            .context("Could not get metadata to keep")?;
        keep_files.extend(metadata_to_keep);
        if let Some(qr) = all_specs.get(&chain.portal_id()) {
            keep_files.insert(qr.to_owned());
        }
    }
    Ok(keep_files)
}
