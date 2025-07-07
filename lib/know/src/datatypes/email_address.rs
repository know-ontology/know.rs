// This is free and unencumbered software released into the public domain.

use alloc::fmt;

/// See: https://datatracker.ietf.org/doc/html/rfc5322#section-3.6
#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmailAddress(String);

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
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
            }
            _ => Err(()),
        }
    }
}
