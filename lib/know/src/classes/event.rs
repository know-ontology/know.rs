// This is free and unencumbered software released into the public domain.

use super::ThingLike;
use crate::{datatypes::DateTime, prelude::*};
use std::{
    fmt::{Debug, Display, Formatter},
    rc::Rc,
};

pub trait EventLike: ThingLike {}

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Event {
    pub name: Option<Name>,

    pub start: Option<DateTime>,

    pub end: Option<DateTime>,
    // TODO: location
    // TODO: organizer
    // TODO: attendees
}

impl ThingLike for Event {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }
}

impl EventLike for Event {}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct EventRef(pub Rc<Event>);

impl ThingLike for EventRef {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> Option<&Name> {
        self.0.name()
    }
}

impl Debug for EventRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = &mut f.debug_struct("EventRef");
        if let Some(name) = self.name() {
            result = result.field("name", name);
        }
        if let Some(ref date) = self.0.start {
            result = result.field("start", date);
        }
        if let Some(ref date) = self.0.end {
            result = result.field("end", date);
        }
        result.finish()
    }
}

impl Display for EventRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (self.id(), self.name(), self.0.start.as_ref()) {
            (Some(id), Some(name), Some(date)) => write!(f, "{} {} (#{})", date, name, id),
            (Some(id), Some(name), None) => write!(f, "{} (#{})", name, id),
            (Some(id), None, Some(date)) => write!(f, "{} (#{})", date, id),
            (Some(id), None, None) => write!(f, "#{}", id),
            (None, Some(name), Some(date)) => write!(f, "{} {}", date, name),
            (None, Some(name), None) => write!(f, "{}", name),
            (None, None, Some(date)) => write!(f, "{}", date),
            (None, None, None) => write!(f, "↪Event"),
        }
    }
}
