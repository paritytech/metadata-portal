use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::updater::source::UpdateSource;

#[derive(Parser)]
pub(crate) struct Opts {
    /// Path to config file
    #[arg(short, long, default_value = "config.toml")]
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

    /// Verify signed QR codes
    Verify,

    /// Check if deployment is up to date
    CheckDeployment,
}

#[derive(Parser)]
pub(crate) struct UpdateOpts {
    #[arg(short, long, default_value = "node")]
    pub(crate) source: UpdateSource,
}
