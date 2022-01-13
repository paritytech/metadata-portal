use std::path::PathBuf;
use qr_generator::{full_run, config::read_app_config};
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
	/// Path to config file
	#[clap(short, long)]
	config_file: PathBuf,
}

fn main() -> Result<(), String> {
	let args = Args::parse();

	let app_config = match read_app_config(args.config_file) {
		Ok(x) => x,
		Err(e) => return Err(format!("Error reading config file. {}", e)),
	};

	match full_run(app_config) {
		Ok(()) => Ok(()),
		Err(e) => return Err(format!("Application error. {}", e)),
	}
}
