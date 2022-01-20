use std::collections::HashMap;
use std::convert::TryFrom;
use std::{fmt, fs};
use std::path::PathBuf;
use definitions::metadata::MetaValues;
use qrcode_rtx::make_pretty_qr;
use crate::{QrCode, ReactAssetPath, UnexpectedQrFilename};
use crate::error::Error;


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
                    QrFileName {
                    chain: String::from(chain),
                    kind: String::from(kind),
                    version: version.parse()?
                })
            },
            _ => Err(UnexpectedQrFilename(path).show())
        }
    }
}

impl fmt::Display for QrFileName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}_{}.apng", self.chain, self.kind, self.version)
    }
}

pub fn read_qr_dir(result: &mut HashMap<String, QrCode>, dir: &PathBuf, public_dir: &PathBuf, is_verified: bool) -> anyhow::Result<()>{
    for file in fs::read_dir(dir)? {
        let path = file?.path();
        let qr_file = QrFileName::try_from(path.clone())?;

        match result.get(&qr_file.chain).map(|qr| qr.version) {
            Some(newest_version) if newest_version >= qr_file.version => (),
            _ => {
                result.insert(String::from(qr_file.chain), QrCode {
                    path: ReactAssetPath::from_fs_path(path, public_dir)?,
                    is_verified,
                    version: qr_file.version,
                });
            }
        }
    }
    Ok(())
}

pub fn generate_metadata_qr(meta_values: &MetaValues, target_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    let crypto_type_code = "ff";
    let prelude = format!("53{}{}", crypto_type_code, "80");

    let file_name = QrFileName{
        chain: meta_values.name.clone(),
        kind: String::from("metadata"),
        version: meta_values.version
    }.to_string();
    let path = target_dir.join(file_name);

    println!("generating QR for {}. It takes a while...", meta_values.name);
    let complete_message = [hex::decode(prelude).expect("known value"), meta_values.meta.clone()].concat();
    if let Err(e) = make_pretty_qr(&complete_message, &path.to_str().unwrap()) {
        return Err(Error::Qr(e.to_string()).show())
    }
    Ok(path)
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
    fn to_string() {
        let obj = QrFileName{
            chain: "chain".to_string(),
            kind: "metadata".to_string(),
            version: 9000,
        };
        assert_eq!(obj.to_string(), format!("{}_{}_{}.apng", obj.chain, obj.kind, obj.version));
    }
}
