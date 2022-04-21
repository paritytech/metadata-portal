use std::path::PathBuf;

use app_config::read_app_config;
use clap::Parser;
use verifier::validate_signed_qrs;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Cli {
    /// Path to config file
    #[clap(short, long, parse(from_os_str))]
    config: PathBuf,
}

fn main() -> Result<(), String> {
    let args = Cli::parse();

    let app_config = match read_app_config(args.config) {
        Ok(x) => x,
        Err(e) => return Err(format!("🔍 Error reading config file. {}", e)),
    };

    match validate_signed_qrs(&app_config.qr_dir, &app_config.verifier.public_key) {
        Ok(_) => println!("Ok!"),
        Err(e) => return Err(format!("❌ Failed to verify: {}", e)),
    }

    Ok(())
}
