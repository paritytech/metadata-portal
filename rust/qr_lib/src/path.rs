use std::convert::TryFrom;
use std::{fmt};
use std::path::PathBuf;
use anyhow::{bail};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct QrPath{
    pub dir: PathBuf,
    pub file_name: QrFileName,
}

impl QrPath {
    pub fn to_path_buf(&self) -> PathBuf {
        self.dir.join(&self.file_name.to_string())
    }
}

impl TryFrom<&PathBuf> for QrPath {
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        Ok(
            Self {
                dir: path.parent().unwrap().to_path_buf(),
                file_name: QrFileName::try_from(path)?
            }
        )
    }
}

impl fmt::Display for QrPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.dir.join(&self.file_name.to_string()).to_str().unwrap())
    }
}


#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct QrFileName{
    pub chain: String,
    pub kind: String,
    pub version: u32,
    pub is_signed: bool,
    extension: Option<String>
}

impl QrFileName{
    const UNSIGNED_PREFIX: &'static str = "unsigned_";

    pub fn new(chain: &str, version: u32, is_signed: bool) -> Self {
        QrFileName{
            chain: chain.to_owned(),
            version,
            kind: "metadata".to_owned(),
            is_signed,
            extension: Some("apng".to_owned())
        }
    }
}

impl TryFrom<&PathBuf> for QrFileName {
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let extension = path.extension().map(|s| s.to_str().unwrap().to_owned());
        let filename = path.file_stem().unwrap().to_str().unwrap();

        let (stripped, is_signed) = match filename.strip_prefix(QrFileName::UNSIGNED_PREFIX) {
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
                        is_signed,
                        extension
                    })
            },
            _ => bail!("QR filename does not follow the format <chain>_<kind>_<version>")
        }
    }
}

impl fmt::Display for QrFileName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = match self.is_signed {
            false => QrFileName::UNSIGNED_PREFIX,
            true => ""
        };
        let file_name = format!("{}{}_{}_{}", prefix, self.chain, self.kind, self.version);

        match &self.extension {
            Some(ext) => write!(f, "{}.{}", file_name, ext),
            None => write!(f, "{}", file_name)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_qr_path() {
        let path = PathBuf::from("./foo/bar/name_metadata_9123.apng");
        let result = QrPath::try_from(&path);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.dir, PathBuf::from("./foo/bar/"));
        assert_eq!(parsed.file_name, QrFileName::new("name", 9123, true));
        assert_eq!(parsed.to_path_buf(), path)
    }

    #[test]
    fn parse_unsigned_qr() {
        let path = PathBuf::from("./foo/bar/unsigned_polkadot_metadata_9123.apng");
        let parse_result = QrFileName::try_from(&path);
        assert!(parse_result.is_ok());
        assert_eq!(parse_result.unwrap(), QrFileName::new("polkadot", 9123, false))
    }

    #[test]
    fn parse_invalid_filename() {
        let path = PathBuf::from("./foo/bar/invalid_9123.apng");
        let parse_result = QrFileName::try_from(&path);
        assert!(parse_result.is_err());
    }

    #[test]
    fn qr_signed_to_string() {
        let obj = QrFileName::new("chain", 9000, true);
        assert_eq!(obj.to_string(), format!("{}_{}_{}.apng", obj.chain, obj.kind, obj.version));
    }

    #[test]
    fn qr_unsigned_to_string() {
        let obj = QrFileName::new("chain", 9000, false);
        assert_eq!(obj.to_string(), format!("unsigned_{}_{}_{}.apng", obj.chain, obj.kind, obj.version));
    }
}
