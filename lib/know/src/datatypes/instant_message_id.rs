// This is free and unencumbered software released into the public domain.

use crate::{formatters::DisplayInline, traits};
use alloc::{borrow::Cow, fmt, str::FromStr};

/// See: https://datatracker.ietf.org/doc/html/rfc5322#section-3.6
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InstantMessageId(String);

impl InstantMessageId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn inline(&self) -> DisplayInline<InstantMessageId> {
        DisplayInline(self)
    }
}

impl fmt::Display for InstantMessageId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for DisplayInline<'_, InstantMessageId> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "🆔 {}", self.0.as_str())
    }
}

#[cfg(feature = "serde")]
impl traits::ToJsonLd for InstantMessageId {
    fn to_jsonld(&self) -> serde_json::Result<serde_json::Value> {
        Ok(format!("mid:{}", self.0).into())
    }
}

impl FromStr for InstantMessageId {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(InstantMessageId(input.to_string()))
    }
}

impl From<String> for InstantMessageId {
    fn from(input: String) -> Self {
        Self(input)
    }
}

impl From<&str> for InstantMessageId {
    fn from(input: &str) -> Self {
        Self(input.to_string())
    }
}

impl From<Cow<'_, str>> for InstantMessageId {
    fn from(input: Cow<str>) -> Self {
        match input {
            Cow::Borrowed(s) => Self(s.to_string()),
            Cow::Owned(s) => Self(s),
        }
    }
}

impl From<&Cow<'_, str>> for InstantMessageId {
    fn from(input: &Cow<str>) -> Self {
        match input {
            Cow::Borrowed(s) => Self(s.to_string()),
            Cow::Owned(s) => Self(s.clone()),
        }
    }
}

impl From<&Cow<'_, [u8]>> for InstantMessageId {
    fn from(input: &Cow<[u8]>) -> Self {
        Self(String::from_utf8_lossy(input).into_owned())
    }
}

impl Into<String> for InstantMessageId {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for InstantMessageId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
