// This is free and unencumbered software released into the public domain.

use crate::formatters::DisplayInline;
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
                        return Ok(EmailAddress(email.to_string()));
                    }
                }
            }
        }

        // If no angle brackets or parsing failed, treat the whole input as email:
        if !trimmed.is_empty() {
            Ok(EmailAddress(trimmed.to_string()))
        } else {
            Err(())
        }
    }
}

#[cfg(feature = "imap-proto")]
impl TryFrom<&imap_proto::Address<'_>> for EmailAddress {
    type Error = ();

    fn try_from(input: &imap_proto::Address) -> Result<Self, Self::Error> {
        match (&input.mailbox, &input.host) {
            (Some(mailbox), Some(host)) => {
                let mailbox = String::from_utf8_lossy(&mailbox);
                let host = String::from_utf8_lossy(&host);
                Ok(Self(format!("{}@{}", mailbox, host)))
            },
            _ => Err(()),
        }
    }
}

#[cfg(feature = "mail-parser")]
impl TryFrom<&mail_parser::Addr<'_>> for EmailAddress {
    type Error = ();

    fn try_from(input: &mail_parser::Addr) -> Result<Self, Self::Error> {
        Ok(Self(
            input
                .address
                .as_ref()
                .map(ToString::to_string)
                .ok_or_else(|| ())?,
        ))
    }
}

#[cfg(feature = "mailparse")]
impl TryFrom<&mailparse::MailAddr> for EmailAddress {
    type Error = mailparse::MailParseError;

    fn try_from(input: &mailparse::MailAddr) -> Result<Self, Self::Error> {
        match input {
            mailparse::MailAddr::Single(single) => Ok(Self(single.addr.clone())),
            mailparse::MailAddr::Group(_group) => todo!(),
        }
    }
}

#[cfg(feature = "mailparse")]
impl TryFrom<&mailparse::MailHeader<'_>> for EmailAddress {
    type Error = mailparse::MailParseError;

    fn try_from(input: &mailparse::MailHeader) -> Result<Self, Self::Error> {
        use mailparse::MailParseError;
        (input.get_value_utf8()?)
            .parse()
            .map_err(|_| MailParseError::Generic("invalid email address in header"))
    }
}
