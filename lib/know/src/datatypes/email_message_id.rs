// This is free and unencumbered software released into the public domain.

use alloc::{borrow::Cow, fmt, str::FromStr};

/// See: https://datatracker.ietf.org/doc/html/rfc5322#section-3.6
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmailMessageId(String);

impl EmailMessageId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EmailMessageId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "🆔 {}", self.0)
    }
}

impl FromStr for EmailMessageId {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(input.to_string()))
    }
}

impl From<String> for EmailMessageId {
    fn from(input: String) -> Self {
        Self(input)
    }
}

impl From<&str> for EmailMessageId {
    fn from(input: &str) -> Self {
        Self(input.to_string())
    }
}

impl From<Cow<'_, str>> for EmailMessageId {
    fn from(input: Cow<str>) -> Self {
        match input {
            Cow::Borrowed(s) => Self(s.to_string()),
            Cow::Owned(s) => Self(s),
        }
    }
}

impl From<&Cow<'_, str>> for EmailMessageId {
    fn from(input: &Cow<str>) -> Self {
        match input {
            Cow::Borrowed(s) => Self(s.to_string()),
            Cow::Owned(s) => Self(s.clone()),
        }
    }
}

impl From<&Cow<'_, [u8]>> for EmailMessageId {
    fn from(input: &Cow<[u8]>) -> Self {
        Self(String::from_utf8_lossy(input).into_owned())
    }
}

impl Into<String> for EmailMessageId {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for EmailMessageId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
