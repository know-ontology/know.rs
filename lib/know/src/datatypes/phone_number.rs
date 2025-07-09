// This is free and unencumbered software released into the public domain.

use crate::formatters::DisplayInline;
use alloc::{borrow::Cow, fmt};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn inline(&self) -> DisplayInline<PhoneNumber> {
        DisplayInline(self)
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for DisplayInline<'_, PhoneNumber> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "📞 {}", self.0.as_str())
    }
}

impl From<String> for PhoneNumber {
    fn from(input: String) -> Self {
        Self(input)
    }
}

impl From<&str> for PhoneNumber {
    fn from(input: &str) -> Self {
        Self(input.to_string())
    }
}

impl From<Cow<'_, str>> for PhoneNumber {
    fn from(input: Cow<str>) -> Self {
        match input {
            Cow::Borrowed(s) => Self(s.to_string()),
            Cow::Owned(s) => Self(s),
        }
    }
}

impl From<&Cow<'_, str>> for PhoneNumber {
    fn from(input: &Cow<str>) -> Self {
        match input {
            Cow::Borrowed(s) => Self(s.to_string()),
            Cow::Owned(s) => Self(s.clone()),
        }
    }
}

impl From<&Cow<'_, [u8]>> for PhoneNumber {
    fn from(input: &Cow<[u8]>) -> Self {
        Self(String::from_utf8_lossy(input).into_owned())
    }
}

impl Into<String> for PhoneNumber {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for PhoneNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
