// This is free and unencumbered software released into the public domain.

use crate::{formatters::DisplayInline, traits};
use alloc::{borrow::Cow, fmt, str::FromStr};

/// See: https://datatracker.ietf.org/doc/html/rfc5322#section-3.6
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmailMessageId(String);

impl EmailMessageId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn inline(&self) -> DisplayInline<EmailMessageId> {
        DisplayInline(self)
    }
}

impl fmt::Display for EmailMessageId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for DisplayInline<'_, EmailMessageId> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "🆔 {}", self.0.as_str())
    }
}

#[cfg(feature = "serde")]
impl traits::ToJsonLd for EmailMessageId {
    fn to_jsonld(&self) -> serde_json::Result<serde_json::Value> {
        Ok(format!("mid:{}", self.0).into())
    }
}

impl FromStr for EmailMessageId {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = input.trim();

        // Check if the input starts and ends with '<' and '>':
        if trimmed.starts_with('<') && trimmed.ends_with('>') && trimmed.len() > 2 {
            let message_id = &trimmed[1..trimmed.len() - 1];
            if !message_id.is_empty() {
                return Ok(EmailMessageId(message_id.to_string()));
            }
        }

        // If no angle brackets or parsing failed, return error:
        Err(())
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
