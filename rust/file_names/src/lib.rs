use std::convert::TryFrom;
use std::{fmt};
use std::path::PathBuf;
use anyhow::{bail};


#[derive(Debug, PartialEq)]
pub struct QrFileName{
    pub chain: String,
    pub kind: String,
    pub version: u32,
}

impl TryFrom<PathBuf> for QrFileName {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        let mut split = file_name.split('_');
        match (split.next(), split.next(), split.next()) {
            (Some(chain), Some(kind), Some(version)) => {
                Ok(
                    Self {
                        chain: String::from(chain),
                        kind: String::from(kind),
                        version: version.parse()?
                    })
            },
            _ => bail!("QR filename does not follow the format <chain>_<kind>_<version>")
        }
    }
}

impl fmt::Display for QrFileName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}_{}.apng", self.chain, self.kind, self.version)
    }
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn parse_valid_qr_filename() {
        let path = Path::new("./foo/bar/name_kind_9123.apng");
        let parse_result = QrFileName::try_from(path.to_path_buf());
        assert!(parse_result.is_ok());
        assert_eq!(parse_result.unwrap(), QrFileName{
            chain: "name".to_string(),
            kind: "kind".to_string(),
            version: 9123,
        })
    }

    #[test]
    fn qr_filename_to_string() {
        let obj = QrFileName{
            chain: "chain".to_string(),
            kind: "metadata".to_string(),
            version: 9000,
        };
        assert_eq!(obj.to_string(), format!("{}_{}_{}.apng", obj.chain, obj.kind, obj.version));
    }
}
