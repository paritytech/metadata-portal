use std::thread;
use anyhow;
use definitions::metadata::MetaValues;

mod helpers;
mod metadata_shortcut;
    use metadata_shortcut::meta_shortcut;
use qrcode_rtx::make_pretty_qr;
use crate::config::AppConfig;

mod error;
pub mod config;

use crate::error::Error;
use crate::metadata_shortcut::fetch_chain_info;


pub fn full_run(app_config: AppConfig) -> anyhow::Result<()> {
    for chain in app_config.chains {
        let meta_specs = fetch_chain_info(&chain.url)?;
        if let Err(e) = generate_metadata_qr(meta_specs.meta_values) {
            eprintln!("Error generating QR for {}: {}", chain.name, e)
        }
        println!("{:?}", meta_specs.specs);
    }


    // for chain in app_config.chains {
    //     let handle = thread::spawn(move || {
    //         if let Err(e) = generate_metadata_qr(&chain) {
    //             eprintln!("Error generating QR for {}: {}", chain.name, e)
    //         }
    //     });
    //     handlers.push(handle);
    // }
    //
    // while let Some(handle) = handlers.pop() {
    //     handle.join().unwrap();
    // }
    Ok(())
}

pub fn generate_metadata_qr(meta_values: MetaValues) -> anyhow::Result<()> {
    // println!("Fetching metadata for {} from {}", chain.name, chain.url);
    // let shortcut = meta_shortcut(&chain.url)?;

    let crypto_type_code = "ff";
    let prelude = format!("53{}{}", crypto_type_code, "80");
    let output_name= format!("unsigned_qr/{}_{}.png", meta_values.name, meta_values.version);
    println!("generating QR for {}. It takes a while...", meta_values.name);
    let complete_message = [hex::decode(prelude).expect("known value"), meta_values.meta].concat();
    if let Err(e) = make_pretty_qr(&complete_message, &output_name) {
        return Err(Error::Qr(e.to_string()).show())
    }
    Ok(())
}
