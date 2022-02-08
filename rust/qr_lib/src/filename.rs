use std::convert::TryFrom;
use std::{fmt};
use std::path::PathBuf;
use anyhow::{bail};

const UNSIGNED_PREFIX: &'static str = "unsigned_";

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct QrFileName{
    pub chain: String,
    pub kind: String,
    pub version: u32,
    pub is_signed: bool,
}

impl TryFrom<&PathBuf> for QrFileName {
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let filename = path.file_stem().unwrap().to_str().unwrap();

        let (stripped, is_signed) = match filename.strip_prefix(UNSIGNED_PREFIX) {
            Some(s) => (s, false),
            None => (filename, true)
        };

        let mut split = stripped.split('_');
        match (split.next(), split.next(), split.next()) {
            (Some(chain), Some(kind), Some(version)) => {
                Ok(
                    Self {
                        chain: String::from(chain),
                        kind: String::from(kind),
                        version: version.parse()?,
                        is_signed
                    })
            },
            _ => bail!("QR filename does not follow the format <chain>_<kind>_<version>")
        }
    }
}

impl fmt::Display for QrFileName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = match self.is_signed {
            false => UNSIGNED_PREFIX,
            true => ""
        };
        write!(f, "{}{}_{}_{}.apng", prefix, self.chain, self.kind, self.version)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_filename() {
        let path = PathBuf::from("./foo/bar/name_kind_9123.apng");
        let parse_result = QrFileName::try_from(&path);
        assert!(parse_result.is_ok());
        assert_eq!(parse_result.unwrap(), QrFileName{
            chain: "name".to_string(),
            kind: "kind".to_string(),
            version: 9123,
            is_signed: true,
        })
    }

    #[test]
    fn parse_unsigned_qr() {
        let path = PathBuf::from("./foo/bar/unsigned_polkadot_metadata_9123.apng");
        let parse_result = QrFileName::try_from(&path);
        assert!(parse_result.is_ok());
        assert_eq!(parse_result.unwrap(), QrFileName{
            chain: "polkadot".to_string(),
            kind: "metadata".to_string(),
            version: 9123,
            is_signed: false,
        })
    }

    #[test]
    fn parse_invalid_filename() {
        let path = PathBuf::from("./foo/bar/invalid_9123.apng");
        let parse_result = QrFileName::try_from(&path);
        assert!(parse_result.is_err());
    }

    #[test]
    fn qr_signed_to_string() {
        let obj = QrFileName{
            chain: "chain".to_string(),
            kind: "metadata".to_string(),
            version: 9000,
            is_signed: true,
        };
        assert_eq!(obj.to_string(), format!("{}_{}_{}.apng", obj.chain, obj.kind, obj.version));
    }

    #[test]
    fn qr_unsigned_to_string() {
        let obj = QrFileName{
            chain: "chain".to_string(),
            kind: "metadata".to_string(),
            version: 9000,
            is_signed: false,
        };
        assert_eq!(obj.to_string(), format!("unsigned_{}_{}_{}.apng", obj.chain, obj.kind, obj.version));
    }
}
