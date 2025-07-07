// This is free and unencumbered software released into the public domain.

use crate::{classes::*, prelude::*};
use std::fmt::{Debug, Formatter};

pub trait ThingLike {
    fn id(&self) -> Option<&str>;
    fn name(&self) -> Option<&Name>;
}

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Thing {
    EmailMessage(EmailMessage),
    Event(Event),
    File(File),
    Group(Group),
    Link(Link),
    Person(Person),
    Place(Place),
    Project(Project),
    SoftwarePackage(SoftwarePackage),
    SoftwareProject(SoftwareProject),
    SoftwareRelease(SoftwareRelease),
}

impl Debug for Thing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Thing::EmailMessage(x) => x.fmt(f),
            Thing::Event(x) => x.fmt(f),
            Thing::File(x) => x.fmt(f),
            Thing::Group(x) => x.fmt(f),
            Thing::Link(x) => x.fmt(f),
            Thing::Person(x) => x.fmt(f),
            Thing::Place(x) => x.fmt(f),
            Thing::Project(x) => x.fmt(f),
            Thing::SoftwarePackage(x) => x.fmt(f),
            Thing::SoftwareProject(x) => x.fmt(f),
            Thing::SoftwareRelease(x) => x.fmt(f),
            #[allow(unreachable_patterns)]
            _ => todo!(),
        }
    }
}

impl ThingLike for Thing {
    fn id(&self) -> Option<&str> {
        match self {
            Thing::EmailMessage(_e) => todo!(), // TODO
            Thing::Event(e) => e.id(),
            Thing::File(f) => f.id(),
            Thing::Group(g) => g.id(),
            Thing::Link(l) => l.id(),
            Thing::Person(p) => p.id(),
            Thing::Place(p) => p.id(),
            Thing::Project(p) => p.id(),
            Thing::SoftwarePackage(p) => p.id(),
            Thing::SoftwareProject(p) => p.id(),
            Thing::SoftwareRelease(p) => p.id(),
        }
    }

    fn name(&self) -> Option<&Name> {
        match self {
            Thing::EmailMessage(_e) => todo!(), // TODO
            Thing::Event(e) => e.name(),
            Thing::File(f) => f.name(),
            Thing::Group(g) => g.name(),
            Thing::Link(l) => l.name(),
            Thing::Person(p) => p.name(),
            Thing::Place(p) => p.name(),
            Thing::Project(p) => p.name(),
            Thing::SoftwarePackage(p) => p.name(),
            Thing::SoftwareProject(p) => p.name(),
            Thing::SoftwareRelease(p) => p.name(),
        }
    }
}
