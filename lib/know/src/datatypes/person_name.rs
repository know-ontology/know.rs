// This is free and unencumbered software released into the public domain.

use crate::{formatters::DisplayInline, traits};
use alloc::{borrow::Cow, fmt};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PersonName(String);

impl PersonName {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn inline(&self) -> DisplayInline<PersonName> {
        DisplayInline(self)
    }
}

impl fmt::Display for PersonName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for DisplayInline<'_, PersonName> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "🧑 {}", self.0.as_str())
    }
}

#[cfg(feature = "serde")]
impl traits::ToJsonLd for PersonName {
    fn to_jsonld(&self) -> serde_json::Result<serde_json::Value> {
        Ok(self.0.to_string().into())
    }
}

impl From<String> for PersonName {
    fn from(input: String) -> Self {
        Self(input)
    }
}

impl From<&str> for PersonName {
    fn from(input: &str) -> Self {
        Self(input.to_string())
    }
}

impl From<Cow<'_, str>> for PersonName {
    fn from(input: Cow<str>) -> Self {
        match input {
            Cow::Borrowed(s) => Self(s.to_string()),
            Cow::Owned(s) => Self(s),
        }
    }
}

impl From<&Cow<'_, str>> for PersonName {
    fn from(input: &Cow<str>) -> Self {
        match input {
            Cow::Borrowed(s) => Self(s.to_string()),
            Cow::Owned(s) => Self(s.clone()),
        }
    }
}

impl From<&Cow<'_, [u8]>> for PersonName {
    fn from(input: &Cow<[u8]>) -> Self {
        Self(String::from_utf8_lossy(input).into_owned())
    }
}

impl Into<String> for PersonName {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<String> for PersonName {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsRef<str> for PersonName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
