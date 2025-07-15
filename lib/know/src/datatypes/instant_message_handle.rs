// This is free and unencumbered software released into the public domain.

use crate::{formatters::DisplayInline, traits};
use alloc::{fmt, str::FromStr};

use super::PhoneNumber;

#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InstantMessageHandle {
    Phone(PhoneNumber),
    Username(String),
}

impl InstantMessageHandle {
    pub fn as_str(&self) -> &str {
        match self {
            InstantMessageHandle::Phone(phone) => phone.as_str(),
            InstantMessageHandle::Username(user) => user.as_str(),
        }
    }

    pub fn inline(&self) -> DisplayInline<InstantMessageHandle> {
        DisplayInline(self)
    }
}

impl fmt::Display for InstantMessageHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for DisplayInline<'_, InstantMessageHandle> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            InstantMessageHandle::Phone(phone) => write!(f, "{}", phone.inline()),
            InstantMessageHandle::Username(user) => write!(f, "📇 {user}"),
        }
    }
}

#[cfg(feature = "serde")]
impl traits::ToJsonLd for InstantMessageHandle {
    fn to_jsonld(&self) -> serde_json::Result<serde_json::Value> {
        Ok(format!("handle:{self}").into())
    }
}

impl FromStr for InstantMessageHandle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = input.trim();

        if let Some(tel) = trimmed.strip_prefix("tel:") {
            return Ok(InstantMessageHandle::Phone(tel.into()));
        }

        // Otherwise treat the whole input as username:
        if !trimmed.is_empty() {
            Ok(InstantMessageHandle::Username(trimmed.to_string()))
        } else {
            Err(())
        }
    }
}
