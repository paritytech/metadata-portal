use std::path::PathBuf;
use anyhow::anyhow;

pub enum Error {
    NotDecodable(NotDecodable),
    FetchFailed{address: String, error: String},
    BadNetworkProperties{address: String, error: String},
    Qr(String),

    UnexpectedQrFilename(PathBuf)
}

pub enum NotDecodable {
    FetchedMetadata{address: String, error: String},
}


impl Error {
    pub fn show(&self) -> anyhow::Error {
        match &self {
            Error::UnexpectedQrFilename(s) => anyhow!("Unexpected filename in {}. Expected format is <chain>_<kind>_<version>", s.display()),
            Error::NotDecodable(x) => {
                match x {
                    NotDecodable::FetchedMetadata{address, error} => anyhow!("Error decoding metadata fetched by rpc call at {}. {}", address, error),
                }
            },
            Error::FetchFailed{address, error} => anyhow!("Error processing rpc call at {}. {}", address, error),
            Error::BadNetworkProperties{address, error} => anyhow!("Error interpreting network properties fetched by rpc call at {}. {}", address, error),
            Error::Qr(e) => anyhow!("Error generating apng qr code. {}", e),
        }
    }
}


