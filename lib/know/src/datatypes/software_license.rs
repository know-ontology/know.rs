// This is free and unencumbered software released into the public domain.

use std::str::FromStr;

/// See: https://en.wikipedia.org/wiki/Software_license
/// See: https://spdx.org/licenses/
#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SoftwareLicense {
    Unspecified,
    Proprietary,
    #[default]
    Unlicense,
    Other(String),
}

impl FromStr for SoftwareLicense {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "" | "n/a" | "none" | "unknown" | "unspecified" => Ok(SoftwareLicense::Unspecified),
            "proprietary" => Ok(SoftwareLicense::Proprietary),
            "unlicense" | "public domain" => Ok(SoftwareLicense::Unlicense),
            _ => Ok(SoftwareLicense::Other(input.to_string())),
        }
    }
}
