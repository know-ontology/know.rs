// This is free and unencumbered software released into the public domain.

use crate::{
    datatypes::{DateTime, EmailAddress, EmailMessageId},
    formatters::{DisplayConcise, DisplayDetailed, DisplayInline, DisplayMime, DisplayOneliner},
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
        writeln!(f, "\tDate: {}", self.0.date)?;
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
            id: input
                .message_id
                .as_ref()
                .and_then(|header| String::from_utf8_lossy(header).parse().ok()),
            in_reply_to: Default::default(),
            references: Default::default(),
            body: Default::default(),
        })
    }
}

#[cfg(feature = "mail-parser")]
impl TryFrom<&mail_parser::Message<'_>> for EmailMessage {
    type Error = ();

    fn try_from(input: &mail_parser::Message) -> Result<Self, Self::Error> {
        use crate::helpers::EmailAddressList;
        Ok(Self {
            date: input
                .date()
                .expect("message must have a Date header") // FIXME
                .try_into()
                .unwrap(),
            from: input
                .from()
                .and_then(|addresses| EmailAddressList::try_from(addresses).ok())
                .map(EmailAddressList::into_vec)
                .unwrap_or_default(),
            sender: None,
            reply_to: None,
            to: input
                .to()
                .and_then(|addresses| EmailAddressList::try_from(addresses).ok())
                .map(EmailAddressList::into_vec)
                .unwrap_or_default(),
            cc: input
                .cc()
                .and_then(|addresses| EmailAddressList::try_from(addresses).ok())
                .map(EmailAddressList::into_vec)
                .unwrap_or_default(),
            bcc: Default::default(),
            subject: input.subject().map(ToString::to_string),
            id: input.message_id().map(EmailMessageId::from),
            in_reply_to: Default::default(),
            references: Default::default(),
            body: Default::default(),
        })
    }
}

#[cfg(feature = "maildir")]
impl TryFrom<&mut maildir::MailEntry> for EmailMessage {
    type Error = maildir::MailEntryError;

    fn try_from(input: &mut maildir::MailEntry) -> Result<Self, Self::Error> {
        Ok((&input.headers()?).try_into()?)
    }
}

#[cfg(feature = "mailparse")]
impl TryFrom<&mailparse::ParsedMail<'_>> for EmailMessage {
    type Error = mailparse::MailParseError;

    fn try_from(input: &mailparse::ParsedMail) -> Result<Self, Self::Error> {
        (&input.headers).try_into()
    }
}

#[cfg(feature = "mailparse")]
impl TryFrom<&Vec<mailparse::MailHeader<'_>>> for EmailMessage {
    type Error = mailparse::MailParseError;

    fn try_from(input: &Vec<mailparse::MailHeader<'_>>) -> Result<Self, Self::Error> {
        use mailparse::{MailHeaderMap, MailParseError};
        Ok(Self {
            date: match input.get_first_header("Date") {
                None => Err(MailParseError::Generic("missing Date header"))?,
                Some(header) => header
                    .try_into()
                    .map_err(|_| MailParseError::Generic("invalid Date header"))?,
            },
            from: input
                .get_all_headers("From")
                .into_iter()
                .filter_map(|header| header.try_into().ok())
                .collect(),
            sender: input
                .get_first_value("Sender")
                .and_then(|header| header.parse().ok()),
            reply_to: input
                .get_first_value("Reply-To")
                .and_then(|header| header.parse().ok()),
            to: input
                .get_all_headers("To")
                .into_iter()
                .filter_map(|header| header.try_into().ok())
                .collect(),
            cc: input
                .get_all_headers("Cc")
                .into_iter()
                .filter_map(|header| header.try_into().ok())
                .collect(),
            bcc: Default::default(),
            subject: input.get_first_value("Subject"),
            id: input
                .get_first_value("Message-ID")
                .and_then(|header| header.parse().ok()),
            in_reply_to: Default::default(),
            references: Default::default(),
            body: Default::default(),
        })
    }
}
