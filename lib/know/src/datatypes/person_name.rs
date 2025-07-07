// This is free and unencumbered software released into the public domain.

use alloc::{borrow::Cow, fmt};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PersonName(String);

impl PersonName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PersonName {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "🧑 {}", self.0)
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
