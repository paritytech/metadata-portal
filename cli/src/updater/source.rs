use anyhow::anyhow;

use std::str::FromStr;

/// The source of the metadata. It can come from the running rcp node or from Github releases.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum UpdateSource {
    Node,
    Github,
}

// impl Display for Source {
//     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Source::File(f) => write!(fmt, "{:?}", f),
//             Source::Chain(c) => write!(fmt, "{:?}", c),
//         }
//     }
// }

impl FromStr for UpdateSource {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "node" => Ok(Self::Node),
            "github" => Ok(Self::Github),
            _ => Err(anyhow!("Invalid source: {}", s)),
        }
    }
}
