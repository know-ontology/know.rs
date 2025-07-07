// This is free and unencumbered software released into the public domain.

use super::{PersonRef, ProjectLike, SoftwareLicense, SoftwarePackageRef, ThingLike};
use crate::{datatypes::EmailAddress, prelude::*};
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde_with::serde_as;

/// See: https://en.wikipedia.org/wiki/Software
#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoftwareProject {
    pub name: Name,

    #[cfg_attr(feature = "serde", serde(default))]
    pub version: String,

    #[cfg_attr(feature = "serde", serde(default))]
    pub summary: String,

    #[cfg_attr(feature = "serde", serde(default))]
    pub description: String,

    #[cfg_attr(feature = "serde", serde(default))]
    pub license: SoftwareLicense,

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "author"),
        serde_as(
            as = "serde_with::OneOrMany<serde_with::PickFirst<(_, serde_with::DisplayFromStr)>>"
        )
    )]
    pub authors: Vec<PersonRef>,

    pub email: Option<EmailAddress>,

    pub link: Option<IRI>,

    pub github: Option<IRI>,

    pub package: Option<SoftwarePackageRef>,
}

impl ThingLike for SoftwareProject {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> Option<&Name> {
        Some(&self.name)
    }
}

impl ProjectLike for SoftwareProject {}

impl FromStr for SoftwareProject {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(SoftwareProject {
            name: input.to_string(),
            ..Default::default()
        })
    }
}
