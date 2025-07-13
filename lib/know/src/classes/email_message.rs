// This is free and unencumbered software released into the public domain.

use crate::{
    datatypes::{DateTime, EmailAddress, EmailMessageId},
    formatters::{
        DisplayConcise, DisplayDetailed, DisplayInline, DisplayJsonLd, DisplayMime, DisplayOneliner,
    },
    traits,
};
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
    pub body: Option<String>,
}

impl EmailMessage {
    pub fn inline(&self) -> DisplayInline<EmailMessage> {
        DisplayInline(self)
    }

    pub fn oneliner(&self) -> DisplayOneliner<EmailMessage> {
        DisplayOneliner(self)
    }

    pub fn concise(&self) -> DisplayConcise<EmailMessage> {
        DisplayConcise(self)
    }

    pub fn detailed(&self) -> DisplayDetailed<EmailMessage> {
        DisplayDetailed(self)
    }

    pub fn mime(&self) -> DisplayMime<EmailMessage> {
        DisplayMime(self)
    }

    pub fn jsonld(&self) -> DisplayJsonLd<EmailMessage> {
        DisplayJsonLd(self)
    }
}

impl fmt::Display for EmailMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.oneliner())
    }
}

impl fmt::Display for DisplayInline<'_, EmailMessage> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.date)?;
        for addr in &self.0.from {
            write!(f, " {}:", addr.as_str())?;
        }
        if let Some(ref subject) = self.0.subject {
            write!(f, " {}", subject)?;
        }
        Ok(())
    }
}

impl fmt::Display for DisplayOneliner<'_, EmailMessage> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.0.inline())
    }
}

impl fmt::Display for DisplayDetailed<'_, EmailMessage> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref subject) = self.0.subject {
            writeln!(f, "✉️  {}", subject)?;
        }
        writeln!(f, "\tDate: {}", self.0.date.inline())?;
        for addr in &self.0.from {
            writeln!(f, "\tFrom: {}", addr)?;
        }
        for addr in &self.0.to {
            writeln!(f, "\tTo: {}", addr)?;
        }
        for addr in &self.0.cc {
            writeln!(f, "\tCc: {}", addr)?;
        }
        for addr in &self.0.bcc {
            writeln!(f, "\tBcc: {}", addr)?;
        }
        if let Some(ref id) = self.0.id {
            writeln!(f, "\tMessage-ID: {}", id)?;
        }
        if let Some(ref in_reply_to) = self.0.in_reply_to {
            writeln!(f, "\tIn-Reply-To: {}", in_reply_to)?;
        }
        if let Some(ref references) = self.0.references {
            writeln!(f, "\tReferences: {}", references)?;
        }
        Ok(())
    }
}

impl fmt::Display for DisplayMime<'_, EmailMessage> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Date: {}", self.0.date.mime())?;
        for addr in &self.0.from {
            writeln!(f, "From: {}", addr.as_str())?;
        }
        for addr in &self.0.to {
            writeln!(f, "To: {}", addr.as_str())?;
        }
        for addr in &self.0.cc {
            writeln!(f, "Cc: {}", addr.as_str())?;
        }
        for addr in &self.0.bcc {
            writeln!(f, "Bcc: {}", addr.as_str())?;
        }
        if let Some(ref subject) = self.0.subject {
            writeln!(f, "Subject: {}", subject)?;
        }
        if let Some(ref id) = self.0.id {
            writeln!(f, "Message-ID: <{}>", id.as_str())?;
        }
        if let Some(ref in_reply_to) = self.0.in_reply_to {
            writeln!(f, "In-Reply-To: {}", in_reply_to)?;
        }
        if let Some(ref references) = self.0.references {
            writeln!(f, "References: {}", references)?;
        }
        if let Some(ref body) = self.0.body {
            writeln!(f)?;
            writeln!(f, "{}", body)?;
        }
        Ok(())
    }
}

#[cfg(feature = "imap-proto")]
include!("email_message/imap_proto.rs");

#[cfg(feature = "mail-parser")]
include!("email_message/mail_parser.rs");

#[cfg(feature = "maildir")]
include!("email_message/maildir.rs");

#[cfg(feature = "mailparse")]
include!("email_message/mailparse.rs");

#[cfg(feature = "serde")]
include!("email_message/serde.rs");
