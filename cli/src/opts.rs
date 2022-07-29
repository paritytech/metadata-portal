use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::updater::source::UpdateSource;

#[derive(Parser)]
pub(crate) struct Opts {
    /// Path to config file
    #[clap(short, long, default_value = "config.toml")]
    pub(crate) config: PathBuf,

    #[clap(subcommand)]
    pub(crate) subcmd: SubCommand,
}

/// You can find all available commands below.
#[derive(Subcommand)]
pub(crate) enum SubCommand {
    /// Remove unused QR codes
    Clean,

    /// Generate json data file for frontend
    Collect,

    /// Sign unsigned QR codes.
    Sign,

    /// Check updates
    Update(UpdateOpts),

    // /// Check updates and sign if update found
    // UpdateAndSign(UpdateOpts),

    /// Verify signed QR codes
    Verify,

    /// Check if deployment is up to date
    CheckDeployment,
}

#[derive(Parser)]
// #[clap(source = "Update source (Node or Github). Node by default.",
//   sign = "Sign or not the metadata. False by default (metadata won't be signed)",
//   signing_key = "Private key for signing metadata. Empty by default. Required if signed argument is true.")]
pub(crate) struct UpdateOpts {
    #[clap(short = 's', long, default_value = "node")]
    pub(crate) source: UpdateSource,

    #[clap(long)]
    pub(crate) sign: bool,

    #[clap(long, default_value = "")]
    pub(crate) signing_key: String,
}
