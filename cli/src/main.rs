mod cleaner;
mod collector;
mod config;
mod lib;
mod opts;
mod signer;
mod updater;
mod verifier;

use clap::StructOpt;
use env_logger::Env;
use log::{error, info};

use crate::cleaner::clean;
use crate::collector::collect;
use crate::config::read_app_config;
use crate::signer::sign;
use crate::updater::update;
use crate::verifier::validate_signed_qrs;
use opts::*;

/// Main entry point of the `metadata-cli`
fn main() -> color_eyre::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let opts: Opts = Opts::parse();

    let config = read_app_config(opts.config).expect("Failed to read config file");

    let result = match opts.subcmd {
        SubCommand::Clean => clean(config),
        SubCommand::Collect => collect(config),
        SubCommand::Sign => sign(config),
        SubCommand::Verify => validate_signed_qrs(&config.qr_dir, &config.verifier.public_key),
        SubCommand::Update => update(config),
    };
    match result {
        Ok(_) => {
            info!("Successfully completed");
        }
        Err(e) => {
            error!("{}", e);
        }
    }

    Ok(())
}
