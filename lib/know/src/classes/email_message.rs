// This is free and unencumbered software released into the public domain.

use crate::datatypes::{DateTime, EmailAddress, EmailMessageId};
use alloc::fmt;

/// See: https://datatracker.ietf.org/doc/html/rfc5322#section-3.6
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmailMessage {
    pub date: DateTime,
    pub from: Vec<EmailAddress>,
    pub sender: Option<EmailAddress>,
    pub reply_to: Option<EmailAddress>,
    pub to: Vec<EmailAddress>,
    pub cc: Vec<EmailAddress>,
    pub bcc: Vec<EmailAddress>,
    pub subject: Option<String>,
    pub id: Option<EmailMessageId>,
    pub in_reply_to: Option<EmailMessageId>,
    pub references: Option<EmailMessageId>,
}

impl fmt::Display for EmailMessage {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref subject) = self.subject {
            writeln!(fmt, "✉️  {}", subject)?;
        }
        writeln!(fmt, "\tDate: {}", self.date)?;
        for addr in &self.from {
            writeln!(fmt, "\tFrom: {}", addr)?;
        }
        for addr in &self.to {
            writeln!(fmt, "\tTo: {}", addr)?;
        }
        for addr in &self.cc {
            writeln!(fmt, "\tCc: {}", addr)?;
        }
        for addr in &self.bcc {
            writeln!(fmt, "\tBcc: {}", addr)?;
        }
        if let Some(ref id) = self.id {
            writeln!(fmt, "\tMessage-ID: {}", id)?;
        }
        if let Some(ref in_reply_to) = self.in_reply_to {
            writeln!(fmt, "\tIn-Reply-To: {}", in_reply_to)?;
        }
        if let Some(ref references) = self.references {
            writeln!(fmt, "\tReferences: {}", references)?;
        }
        Ok(())
    }
}

#[cfg(feature = "imap-proto")]
impl TryFrom<&imap_proto::types::Envelope<'_>> for EmailMessage {
    type Error = ();

    fn try_from(input: &imap_proto::types::Envelope) -> Result<Self, Self::Error> {
        let input_date = String::from_utf8_lossy(input.date.as_ref().unwrap());
        let input_date = jiff::fmt::rfc2822::parse(&input_date).map_err(|_| ())?;
        Ok(Self {
            date: input_date.into(),
            from: input
                .from
                .as_ref()
                .map(|xs| xs.into_iter().map(|x| x.try_into().unwrap()).collect())
                .unwrap_or_default(),
            sender: input
                .sender
                .as_ref()
                .map(|xs| xs.into_iter().map(|x| x.try_into().unwrap()).next())
                .unwrap_or_default(),
            reply_to: input
                .reply_to
                .as_ref()
                .map(|xs| xs.into_iter().map(|x| x.try_into().unwrap()).next())
                .unwrap_or_default(),
            to: input
                .to
                .as_ref()
                .map(|xs| xs.into_iter().map(|x| x.try_into().unwrap()).collect())
                .unwrap_or_default(),
            cc: input
                .cc
                .as_ref()
                .map(|xs| xs.into_iter().map(|x| x.try_into().unwrap()).collect())
                .unwrap_or_default(),
            bcc: Default::default(),
            subject: input
                .subject
                .as_ref()
                .map(|x| String::from_utf8_lossy(x).into_owned()),
            id: input.message_id.as_ref().map(|x| x.into()),
            in_reply_to: Default::default(),
            references: Default::default(),
        })
    }
}
