use std::path::PathBuf;
use anyhow::anyhow;

pub enum Error {
    NotDecodeable(NotDecodeable),
    FetchFailed{address: String, error: String},
    BadNetworkProperties{address: String, error: String},
    Qr(String),

    UnexpectedQrFilename(PathBuf)
}

pub enum NotDecodeable {
    FetchedMetadata{address: String, error: String},
}


impl Error {
    pub fn show(&self) -> anyhow::Error {
        match &self {
            Error::UnexpectedQrFilename(s) => anyhow!("Unexpected filename in {}. Expected format is <chain>_<kind>_<version>", s.display()),
            Error::NotDecodeable(x) => {
                match x {
                    NotDecodeable::FetchedMetadata{address, error} => anyhow!("Error decoding metadata fetched by rpc call at {}. {}", address, error),
                }
            },
            Error::FetchFailed{address, error} => anyhow!("Error processing rpc call at {}. {}", address, error),
            Error::BadNetworkProperties{address, error} => anyhow!("Error interpreting network properties fetched by rpc call at {}. {}", address, error),
            Error::Qr(e) => anyhow!("Error generating apng qr code. {}", e),
        }
    }
}


