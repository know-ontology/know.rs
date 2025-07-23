// This is free and unencumbered software released into the public domain.

use crate::{formatters::DisplayInline, traits};
use alloc::{fmt, str::FromStr};

use super::{InstantMessageHandle, PhoneNumber};

#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InstantMessageRecipientType {
    Direct,
    Group,
    Channel,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InstantMessageRecipient {
    Direct(InstantMessageHandle),
    // Group( some kind of group id ),
    // Channel( channel id ),
}

impl InstantMessageRecipient {
    pub fn as_str(&self) -> &str {
        match self {
            InstantMessageRecipient::Direct(handle) => handle.as_str(),
        }
    }

    pub fn inline(&self) -> DisplayInline<InstantMessageRecipient> {
        DisplayInline(self)
    }

    pub fn r#type(&self) -> InstantMessageRecipientType {
        match self {
            InstantMessageRecipient::Direct(_) => InstantMessageRecipientType::Direct,
        }
    }
}

impl fmt::Display for InstantMessageRecipient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for DisplayInline<'_, InstantMessageRecipient> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            InstantMessageRecipient::Direct(direct) => write!(f, "{}", direct.inline()),
        }
    }
}
