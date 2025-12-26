// This is free and unencumbered software released into the public domain.

use super::ThingLike;
use crate::prelude::*;

pub trait GroupLike: ThingLike {}

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(default, tag = "@type", rename_all = "camelCase")
)]
pub struct Group {
    pub name: Name,
}

impl ThingLike for Group {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> Option<&Name> {
        Some(&self.name)
    }
}

impl GroupLike for Group {}
