mod cleaner;
mod collector;
mod config;
mod deployment_checker;
mod export;
mod fetch;
mod lib;
mod opts;
mod qrs;
mod signer;
mod source;
mod updater;
mod verifier;
mod autosigner;

use std::process::exit;

use clap::StructOpt;
use env_logger::Env;
use log::error;

use crate::autosigner::{autosign_from_github, autosign_from_node};
use crate::cleaner::clean;
use crate::collector::collect;
use crate::config::AppConfig;
use crate::deployment_checker::check_deployment;
use crate::fetch::RpcFetcher;
use crate::opts::{Opts, SubCommand};
use crate::signer::sign;
use crate::updater::source::UpdateSource;
use crate::updater::{update_from_github, update_from_node};
use crate::verifier::verify;

/// Main entry point of the `metadata-cli`
fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_target(true)
        .init();

    let opts: Opts = Opts::parse();
    let config = match AppConfig::load(opts.config) {
        Ok(config) => config,
        Err(err) => {
            error!("{}", err);
            exit(1);
        }
    };

    let result = match opts.subcmd {
        SubCommand::Clean => clean(config),
        SubCommand::Collect => collect(config),
        SubCommand::Sign => sign(config),
        SubCommand::AutoSign(autosign_opts) => match autosign_opts.source {
            UpdateSource::Github => autosign_from_github(config),
            UpdateSource::Node => autosign_from_node(config, RpcFetcher),
        },
        SubCommand::Verify => verify(config),
        SubCommand::Update(update_opts) => match update_opts.source {
            UpdateSource::Github => update_from_github(config),
            UpdateSource::Node => update_from_node(config, RpcFetcher),
        },
        SubCommand::CheckDeployment => check_deployment(config),
    };

    if let Err(err) = result {
        error!("{}", err);
        exit(1);
    }
}
