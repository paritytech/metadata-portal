use std::convert::TryFrom;
use std::fmt;
use std::path::PathBuf;

use anyhow::{bail, Context};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub(crate) struct QrPath {
    pub(crate) dir: PathBuf,
    pub(crate) file_name: QrFileName,
}

impl QrPath {
    pub(crate) fn to_path_buf(&self) -> PathBuf {
        self.dir.join(self.file_name.to_string())
    }
}

impl TryFrom<&PathBuf> for QrPath {
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        Ok(Self {
            dir: path.parent().unwrap().to_path_buf(),
            file_name: QrFileName::try_from(path)?,
        })
    }
}

impl fmt::Display for QrPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.dir.join(self.file_name.to_string()).to_str().unwrap()
        )
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub(crate) enum ContentType {
    Metadata(u32),
    Specs,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContentType::Metadata(version) => write!(f, "metadata_{version}"),
            ContentType::Specs => write!(f, "specs"),
        }
    }
}

impl TryFrom<&str> for ContentType {
    type Error = anyhow::Error;

    fn try_from(content_type: &str) -> Result<Self, Self::Error> {
        if content_type.ends_with("specs") {
            return Ok(Self::Specs);
        }
        let mut split = content_type.rsplit('_');
        match (split.next(), split.next()) {
            (Some(version), Some("metadata")) => Ok(Self::Metadata(version.parse()?)),
            _ => bail!("unable to parse content type {}", content_type),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub(crate) struct QrFileName {
    pub(crate) chain: String,
    pub(crate) is_signed: bool,
    pub(crate) content_type: ContentType,
    extension: Option<String>,
}

impl QrFileName {
    const UNSIGNED_PREFIX: &'static str = "unsigned_";

    pub(crate) fn new(chain: &str, content_type: ContentType, is_signed: bool) -> Self {
        let extension = match content_type {
            ContentType::Metadata(_) => "apng",
            ContentType::Specs => "png",
        };

        QrFileName {
            chain: chain.to_owned(),
            content_type,
            is_signed,
            extension: Some(extension.to_string()),
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
            None => (filename, true),
        };

        let content_type = ContentType::try_from(stripped).context("error parsing context type")?;
        let chain = stripped
            .strip_suffix(&format!("_{content_type}"))
            .context("error parsing chain name")?;

        Ok(Self {
            chain: String::from(chain),
            content_type,
            is_signed,
            extension,
        })
    }
}

impl fmt::Display for QrFileName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = match self.is_signed {
            false => QrFileName::UNSIGNED_PREFIX,
            true => "",
        };
        let file_name = format!("{}{}_{}", prefix, self.chain, self.content_type);
        match &self.extension {
            Some(ext) => write!(f, "{file_name}.{ext}"),
            None => write!(f, "{file_name}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_metadata_qr_path() {
        let path = PathBuf::from("./foo/bar/name_metadata_9123.apng");
        let result = QrPath::try_from(&path);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.dir, PathBuf::from("./foo/bar/"));
        assert_eq!(
            parsed.file_name,
            QrFileName::new("name", ContentType::Metadata(9123), true)
        );
        assert_eq!(parsed.to_path_buf(), path)
    }

    #[test]
    fn parse_unsigned_underscored_qr_path() {
        let path = PathBuf::from("./foo/unsigned_name_with_underscore_metadata_91.apng");
        let result = QrPath::try_from(&path);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(
            parsed.file_name,
            QrFileName::new("name_with_underscore", ContentType::Metadata(91), false)
        );
    }

    #[test]
    fn parse_unsigned_metadata_qr() {
        let path = PathBuf::from("./foo/bar/unsigned_polkadot_metadata_9123.apng");
        let parse_result = QrFileName::try_from(&path);
        assert!(parse_result.is_ok());
        assert_eq!(
            parse_result.unwrap(),
            QrFileName::new("polkadot", ContentType::Metadata(9123), false)
        )
    }

    #[test]
    fn parse_invalid_filename() {
        let path = PathBuf::from("./foo/bar/invalid_9123.apng");
        let parse_result = QrFileName::try_from(&path);
        assert!(parse_result.is_err());
    }

    #[test]
    fn qr_signed_metadata_to_string() {
        let obj = QrFileName::new("chain", ContentType::Metadata(9000), true);
        assert_eq!(obj.to_string(), "chain_metadata_9000.apng");
    }

    #[test]
    fn qr_unsigned_to_string() {
        let obj = QrFileName::new("chain", ContentType::Metadata(9000), false);
        assert_eq!(obj.to_string(), "unsigned_chain_metadata_9000.apng");
    }

    #[test]
    fn parse_specs_qr_path() {
        let path = PathBuf::from("./foo/bar/polkadot_specs.png");
        let result = QrPath::try_from(&path);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.dir, PathBuf::from("./foo/bar/"));
        assert_eq!(
            parsed.file_name,
            QrFileName::new("polkadot", ContentType::Specs, true)
        );
        assert_eq!(parsed.to_path_buf(), path)
    }

    #[test]
    fn parse_specs_qr_to_string() {
        let obj = QrFileName::new("polkadot", ContentType::Specs, true);
        assert_eq!(obj.to_string(), "polkadot_specs.png");
    }

    #[test]
    fn parse_unsigned_specs_qr_to_string() {
        let obj = QrFileName::new("polkadot", ContentType::Specs, false);
        assert_eq!(obj.to_string(), "unsigned_polkadot_specs.png");
    }
}
