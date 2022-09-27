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

    /// Verify signed QR codes
    Verify,

    /// Check if deployment is up to date
    CheckDeployment,

    /// Check if the config.toml should be updated
    UpdateChainConfig(ChainsOpts),
}

#[derive(Parser)]
pub(crate) struct UpdateOpts {
    #[clap(short = 's', long, default_value = "node")]
    pub(crate) source: UpdateSource,

    #[clap(long)]
    pub(crate) sign: bool,

    #[clap(long, default_value = "")]
    pub(crate) signing_key: String,
}

#[derive(Parser)]
pub(crate) struct ChainsOpts {
    #[clap(long, default_value = "prod")]
    pub(crate) env: String,

    #[clap(long, default_value = "v5")]
    pub(crate) version: String,
}
