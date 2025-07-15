// This is free and unencumbered software released into the public domain.

use super::ThingLike;
use crate::{
    datatypes::{DateTime, InstantMessageHandle, InstantMessageId, PhoneNumber},
    formatters::{
        DisplayConcise, DisplayDetailed, DisplayInline, DisplayJsonLd, DisplayMime, DisplayOneliner,
    },
    prelude::Name,
    traits,
};
use alloc::fmt;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InstantMessage {
    pub id: Option<InstantMessageId>,
    pub date: DateTime,
    pub sender: InstantMessageHandle,
    pub receiver: InstantMessageHandle,
    pub platform: Option<String>,
    pub references: Option<InstantMessageId>,
    pub content: String,
    // TODO: group context?
}

impl InstantMessage {
    pub fn inline(&self) -> DisplayInline<InstantMessage> {
        DisplayInline(self)
    }

    pub fn oneliner(&self) -> DisplayOneliner<InstantMessage> {
        DisplayOneliner(self)
    }

    pub fn concise(&self) -> DisplayConcise<InstantMessage> {
        DisplayConcise(self)
    }

    pub fn detailed(&self) -> DisplayDetailed<InstantMessage> {
        DisplayDetailed(self)
    }

    pub fn jsonld(&self) -> DisplayJsonLd<InstantMessage> {
        DisplayJsonLd(self)
    }
}

impl fmt::Display for InstantMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.oneliner())
    }
}

impl fmt::Display for DisplayInline<'_, InstantMessage> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.date)?;
        write!(f, " {}:", self.0.sender)?;
        write!(f, " {}", self.0.receiver)?;
        Ok(())
    }
}

impl fmt::Display for DisplayOneliner<'_, InstantMessage> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.0.inline())
    }
}

impl fmt::Display for DisplayDetailed<'_, InstantMessage> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref id) = self.0.id {
            writeln!(f, "📲: {}", id.inline())?;
        }
        writeln!(f, "\tDate: {}", self.0.date.inline())?;
        writeln!(f, "\tSender: {}", self.0.sender.inline())?;
        writeln!(f, "\tReceiver: {}", self.0.receiver.inline())?;
        if let Some(ref platform) = self.0.platform {
            writeln!(f, "\tPlatform: {}", platform)?;
        }
        if let Some(ref references) = self.0.references {
            writeln!(f, "\tReferences: {}", references.inline())?;
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
impl fmt::Display for DisplayJsonLd<'_, InstantMessage> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::traits::ToJsonLd;
        let json = self.0.to_jsonld().unwrap(); // infallible
        if cfg!(feature = "pretty") {
            let mut w = crate::formatters::WriteToFormatter::new(f);
            colored_json::write_colored_json(&json, &mut w).map_err(|_| fmt::Error)?;
            writeln!(f)
        } else {
            writeln!(f, "{}", json)
        }
    }
}

#[cfg(feature = "serde")]
impl traits::ToJsonLd for InstantMessage {
    fn to_jsonld(&self) -> serde_json::Result<serde_json::Value> {
        use serde_json::json;
        Ok(json!({
            "@id": match self.id {
                Some(ref id) => id.to_jsonld()?,
                None => "_:message".into(),
            },
            "@type": "InstantMessage",
            "sender": self.sender.to_jsonld()?,
            "receiver": self.receiver.to_jsonld()?,
            "platform": self.platform,
            "body": self.content,
        }))
    }
}

impl ThingLike for InstantMessage {
    fn id(&self) -> Option<&str> {
        self.id.as_ref().map(|id| id.as_str())
    }

    fn name(&self) -> Option<&Name> {
        None
    }
}
