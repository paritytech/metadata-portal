use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Opts {
    /// Path to config file
    #[clap(short, long, default_value = "config.toml")]
    pub config: PathBuf,

    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

/// You can find all available commands below.
#[derive(Subcommand)]
pub enum SubCommand {
    /// Remove unused QR codes
    Clean,

    /// Generate json data file for frontend
    Collect,

    /// Sign unsigned QR codes.
    Sign,

    /// Check updates on rpc nodes
    Update,

    /// Verify signed QR codes
    Verify,
}
