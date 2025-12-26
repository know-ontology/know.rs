// This is free and unencumbered software released into the public domain.

use crate::{
    datatypes::{DateTime, EmailAddress, EmailMessageId},
    formatters::{
        DisplayConcise, DisplayDetailed, DisplayInline, DisplayJsonLd, DisplayMime, DisplayOneliner,
    },
};
use alloc::fmt;

/// See: https://datatracker.ietf.org/doc/html/rfc5322#section-3.6
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(default, tag = "@type", rename_all = "camelCase")
)]
pub struct EmailMessage {
    /// See: https://datatracker.ietf.org/doc/html/draft-ietf-emailcore-rfc5322bis-12#name-the-origination-date-field
    pub date: DateTime,

    /// See: https://datatracker.ietf.org/doc/html/draft-ietf-emailcore-rfc5322bis-12#name-originator-fields
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub from: Vec<EmailAddress>,

    /// See: https://datatracker.ietf.org/doc/html/draft-ietf-emailcore-rfc5322bis-12#name-originator-fields
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sender: Option<EmailAddress>,

    /// See: https://datatracker.ietf.org/doc/html/draft-ietf-emailcore-rfc5322bis-12#name-originator-fields
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub reply_to: Vec<EmailAddress>,

    /// See: https://datatracker.ietf.org/doc/html/draft-ietf-emailcore-rfc5322bis-12#name-destination-address-fields
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub to: Vec<EmailAddress>,

    /// See: https://datatracker.ietf.org/doc/html/draft-ietf-emailcore-rfc5322bis-12#name-destination-address-fields
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub cc: Vec<EmailAddress>,

    /// See: https://datatracker.ietf.org/doc/html/draft-ietf-emailcore-rfc5322bis-12#name-destination-address-fields
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub bcc: Vec<EmailAddress>,

    /// See: https://datatracker.ietf.org/doc/html/draft-ietf-emailcore-rfc5322bis-12#name-informational-fields
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub subject: Option<String>,

    /// See: https://datatracker.ietf.org/doc/html/draft-ietf-emailcore-rfc5322bis-12#section-3.6.4
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub id: Option<EmailMessageId>,

    /// See: https://datatracker.ietf.org/doc/html/draft-ietf-emailcore-rfc5322bis-12#section-3.6.4
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub in_reply_to: Vec<EmailMessageId>,

    /// See: https://datatracker.ietf.org/doc/html/draft-ietf-emailcore-rfc5322bis-12#section-3.6.4
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub references: Vec<EmailMessageId>,

    /// See: https://datatracker.ietf.org/doc/html/draft-ietf-emailcore-rfc5322bis-12#name-body
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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

impl fmt::Display for DisplayConcise<'_, EmailMessage> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref subject) = self.0.subject {
            writeln!(f, "✉️  {}", subject)?;
        }
        writeln!(f, "\tDate: {}", self.0.date.inline())?;
        if !self.0.from.is_empty() {
            writeln!(
                f,
                "\tFrom: {}",
                &self
                    .0
                    .from
                    .iter()
                    .map(|addr| format!("{}", addr.inline()))
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }
        if !self.0.to.is_empty() {
            writeln!(
                f,
                "\tTo: {}",
                &self
                    .0
                    .to
                    .iter()
                    .map(|addr| format!("{}", addr.inline()))
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }
        if !self.0.cc.is_empty() {
            writeln!(
                f,
                "\tCc: {}",
                &self
                    .0
                    .cc
                    .iter()
                    .map(|addr| format!("{}", addr.inline()))
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }
        if !self.0.bcc.is_empty() {
            writeln!(
                f,
                "\tBcc: {}",
                &self
                    .0
                    .bcc
                    .iter()
                    .map(|addr| format!("{}", addr.inline()))
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }
        if let Some(ref id) = self.0.id {
            writeln!(f, "\tMessage-ID: {}", id.inline())?;
        }
        Ok(())
    }
}

impl fmt::Display for DisplayDetailed<'_, EmailMessage> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref subject) = self.0.subject {
            writeln!(f, "✉️  {}", subject)?;
        }
        writeln!(f, "\tDate: {}", self.0.date.inline())?;
        for addr in &self.0.from {
            writeln!(f, "\tFrom: {}", addr.inline())?;
        }
        for addr in &self.0.to {
            writeln!(f, "\tTo: {}", addr.inline())?;
        }
        for addr in &self.0.cc {
            writeln!(f, "\tCc: {}", addr.inline())?;
        }
        for addr in &self.0.bcc {
            writeln!(f, "\tBcc: {}", addr.inline())?;
        }
        if let Some(ref id) = self.0.id {
            writeln!(f, "\tMessage-ID: {}", id.inline())?;
        }
        for in_reply_to in &self.0.in_reply_to {
            writeln!(f, "\tIn-Reply-To: {}", in_reply_to.inline())?;
        }
        for references in &self.0.references {
            writeln!(f, "\tReferences: {}", references.inline())?;
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
        for in_reply_to in &self.0.in_reply_to {
            writeln!(f, "In-Reply-To: {}", in_reply_to.inline())?;
        }
        for references in &self.0.references {
            writeln!(f, "References: {}", references.inline())?;
        }
        if let Some(ref body) = self.0.body {
            writeln!(f)?;
            writeln!(f, "{}", body)?;
        }
        Ok(())
    }
}

#[cfg(feature = "tldr")]
impl tldr::Tldr for EmailMessage {
    type Error = Box<dyn core::error::Error>;

    fn what(&self, ctx: &tldr::TldrContext) -> tldr::TldrResult<String, Self::Error> {
        use core::fmt::Write;
        use tldr::TldrLanguage::*;
        Ok(match ctx.language {
            English => {
                let timespan = DateTime::now().since(&self.date)?.round()?;

                let mut tldr = String::new();
                write!(tldr, "An email message dated {timespan} ago")?;
                if let Some(from) = &self.from.first() {
                    write!(tldr, ", from {}", from)?;
                }
                if !self.to.is_empty() {
                    write!(tldr, ", addressed to ")?;
                    for (i, addr) in self.to.iter().enumerate() {
                        if i > 0 {
                            write!(tldr, ", ")?;
                        }
                        write!(tldr, "{}", addr)?;
                    }
                }
                if let Some(subject) = &self.subject {
                    write!(tldr, ", with the subject \"{}\"", subject)?;
                }
                Some(tldr)
            },
            _ => None,
        })
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
