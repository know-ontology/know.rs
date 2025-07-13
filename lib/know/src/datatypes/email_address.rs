// This is free and unencumbered software released into the public domain.

use crate::{formatters::DisplayInline, traits};
use alloc::{fmt, str::FromStr};

/// See: https://datatracker.ietf.org/doc/html/rfc5322#section-3.6
#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn inline(&self) -> DisplayInline<EmailAddress> {
        DisplayInline(self)
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for DisplayInline<'_, EmailAddress> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "📧 {}", self.0.as_str())
    }
}

impl FromStr for EmailAddress {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trimmed = input.trim();

        // Check if the input contains angle brackets:
        if let Some(start) = trimmed.rfind('<') {
            if let Some(end) = trimmed.rfind('>') {
                if start < end {
                    let email = trimmed[start + 1..end].trim();
                    if !email.is_empty() {
                        return Ok(EmailAddress(email.to_lowercase()));
                    }
                }
            }
        }

        // If no angle brackets or parsing failed, treat the whole input as email:
        if !trimmed.is_empty() {
            Ok(EmailAddress(trimmed.to_lowercase()))
        } else {
            Err(())
        }
    }
}

impl Into<String> for EmailAddress {
    fn into(self) -> String {
        self.0
    }
}

#[cfg(feature = "imap-proto")]
include!("email_address/imap_proto.rs");

#[cfg(feature = "mail-parser")]
include!("email_address/mail_parser.rs");

#[cfg(feature = "mailparse")]
include!("email_address/mailparse.rs");

#[cfg(feature = "serde")]
include!("email_address/serde.rs");
