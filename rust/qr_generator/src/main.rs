use std::thread;
use qr_generator::{full_run, config::read_app_config};


fn main() -> Result<(), String> {
	let app_config = match read_app_config("config.toml") {
		Ok(x) => x,
		Err(e) => return Err(format!("Error reading config file. {}", e)),
	};

	match full_run(app_config) {
		Ok(()) => Ok(()),
		Err(e) => return Err(format!("Application error. {}", e)),
	}
}
