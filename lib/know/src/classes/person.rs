// This is free and unencumbered software released into the public domain.

use super::{EventRef, ThingLike};
use crate::{
    datatypes::{Age, Date, EmailAddress, PersonName, PhoneNumber},
    prelude::*,
};
use std::{
    fmt::{Debug, Display, Formatter},
    rc::Rc,
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde_with::serde_as;

pub trait PersonLike: ThingLike {
    fn nickname(&self) -> Option<&PersonName>;
    fn nicknames(&self) -> &Vec<PersonName>;
    fn age(&self) -> Option<Age>;
    fn birthdate(&self) -> Option<Date>;
    fn birth(&self) -> Option<&EventRef>;
    fn death(&self) -> Option<&EventRef>;
    fn parents(&self) -> Vec<PersonRef>;
    fn father(&self) -> Option<&PersonRef>;
    fn mother(&self) -> Option<&PersonRef>;
    fn siblings(&self) -> &Vec<PersonRef>;
    fn spouse(&self) -> Option<&PersonRef>;
    fn spouses(&self) -> &Vec<PersonRef>;
    fn partner(&self) -> Option<&PersonRef>;
    fn partners(&self) -> &Vec<PersonRef>;
    fn children(&self) -> &Vec<PersonRef>;
    fn colleagues(&self) -> &Vec<PersonRef>;
    fn knows(&self) -> &Vec<PersonRef>;
    fn email(&self) -> Option<&EmailAddress>;
    fn emails(&self) -> &Vec<EmailAddress>;
    fn phone(&self) -> Option<&PhoneNumber>;
    fn phones(&self) -> &Vec<PhoneNumber>;
    fn account(&self) -> Option<&String>;
    fn accounts(&self) -> &Vec<String>;
    fn link(&self) -> Option<&String>;
    fn links(&self) -> &Vec<String>;
}

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(default, tag = "@type", rename_all = "camelCase")
)]
pub struct Person {
    #[cfg_attr(
        feature = "serde",
        serde(rename = "@id", skip_serializing_if = "Option::is_none")
    )]
    pub id: Option<String>,

    pub name: PersonName, // FIXME: Option<PersonName>

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "nickname", skip_serializing_if = "Vec::is_empty"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub nicknames: Vec<PersonName>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub age: Option<Age>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub birth: Option<EventRef>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub death: Option<EventRef>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub father: Option<PersonRef>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mother: Option<PersonRef>,

    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            alias = "sibling",
            alias = "brothers",
            alias = "brother",
            alias = "sisters",
            alias = "sister",
            skip_serializing_if = "Vec::is_empty"
        ),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub siblings: Vec<PersonRef>,

    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            alias = "spouse",
            alias = "husband",
            alias = "wife",
            skip_serializing_if = "Vec::is_empty"
        ),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub spouses: Vec<PersonRef>,

    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            alias = "partner",
            alias = "boyfriend",
            alias = "girlfriend",
            skip_serializing_if = "Vec::is_empty"
        ),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub partners: Vec<PersonRef>,

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "child", skip_serializing_if = "Vec::is_empty"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub children: Vec<PersonRef>,

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "colleague", skip_serializing_if = "Vec::is_empty"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub colleagues: Vec<PersonRef>,

    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub knows: Vec<PersonRef>,

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "email", skip_serializing_if = "Vec::is_empty"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub emails: Vec<EmailAddress>,

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "phone", skip_serializing_if = "Vec::is_empty"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub phones: Vec<PhoneNumber>,

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "account", skip_serializing_if = "Vec::is_empty"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub accounts: Vec<String>, // TODO: datatype

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "link", skip_serializing_if = "Vec::is_empty"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub links: Vec<String>, // TODO: datatype
}

impl ThingLike for Person {
    fn id(&self) -> Option<&str> {
        self.id.as_ref().map(|s| s.as_ref())
    }

    fn name(&self) -> Option<&Name> {
        Some(&self.name.as_ref())
    }
}

impl PersonLike for Person {
    fn nickname(&self) -> Option<&PersonName> {
        self.nicknames.first()
    }

    fn nicknames(&self) -> &Vec<PersonName> {
        self.nicknames.as_ref()
    }

    fn age(&self) -> Option<Age> {
        self.age // TODO: calculate from self.birthdate
    }

    fn birthdate(&self) -> Option<Date> {
        match self.birth {
            Some(ref _event) => todo!(), // FIXME: event.0.start.as_ref(),
            None => None,
        }
    }

    fn birth(&self) -> Option<&EventRef> {
        self.birth.as_ref()
    }

    fn death(&self) -> Option<&EventRef> {
        self.death.as_ref()
    }

    fn parents(&self) -> Vec<PersonRef> {
        let mut result = vec![];
        if let Some(father) = self.father() {
            result.push(father.clone());
        }
        if let Some(mother) = self.mother() {
            result.push(mother.clone());
        }
        result
    }

    fn father(&self) -> Option<&PersonRef> {
        self.father.as_ref()
    }

    fn mother(&self) -> Option<&PersonRef> {
        self.mother.as_ref()
    }

    fn siblings(&self) -> &Vec<PersonRef> {
        self.siblings.as_ref()
    }

    fn spouse(&self) -> Option<&PersonRef> {
        self.spouses.first()
    }

    fn spouses(&self) -> &Vec<PersonRef> {
        self.spouses.as_ref()
    }

    fn partner(&self) -> Option<&PersonRef> {
        self.partners.first()
    }

    fn partners(&self) -> &Vec<PersonRef> {
        self.partners.as_ref()
    }

    fn children(&self) -> &Vec<PersonRef> {
        self.children.as_ref()
    }

    fn colleagues(&self) -> &Vec<PersonRef> {
        self.colleagues.as_ref()
    }

    fn knows(&self) -> &Vec<PersonRef> {
        self.knows.as_ref()
    }

    fn email(&self) -> Option<&EmailAddress> {
        self.emails.first()
    }

    fn emails(&self) -> &Vec<EmailAddress> {
        self.emails.as_ref()
    }

    fn phone(&self) -> Option<&PhoneNumber> {
        self.phones.first()
    }

    fn phones(&self) -> &Vec<PhoneNumber> {
        self.phones.as_ref()
    }

    fn account(&self) -> Option<&String> {
        self.accounts.first()
    }

    fn accounts(&self) -> &Vec<String> {
        self.accounts.as_ref()
    }

    fn link(&self) -> Option<&String> {
        self.links.first()
    }

    fn links(&self) -> &Vec<String> {
        self.links.as_ref()
    }
}

impl FromStr for Person {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Person {
            name: input.into(),
            ..Default::default()
        })
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PersonRef(pub Rc<Person>);

impl ThingLike for PersonRef {
    fn id(&self) -> Option<&str> {
        self.0.id()
    }

    fn name(&self) -> Option<&Name> {
        self.0.name()
    }
}

impl Debug for PersonRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = &mut f.debug_struct("PersonRef");
        if !self.0.name.as_str().is_empty() {
            result = result.field("name", &self.0.name);
        }
        result = match self.0.nicknames.len() {
            0 => result,
            1 => result.field("nickname", &self.0.nicknames[0]),
            _ => result.field("nicknames", &self.0.nicknames),
        };
        result = match self.0.emails.len() {
            0 => result,
            1 => result.field("email", &self.0.emails[0]),
            _ => result.field("emails", &self.0.emails),
        };
        result.finish()
    }
}

impl Display for PersonRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (self.id(), self.name(), self.0.email()) {
            (Some(id), Some(name), Some(email)) => write!(f, "{} <{}> (#{})", name, email, id),
            (Some(id), Some(name), None) => write!(f, "{} (#{})", name, id),
            (Some(id), None, Some(email)) => write!(f, "<{}> (#{})", email, id),
            (Some(id), None, None) => write!(f, "#{}", id),
            (None, Some(name), Some(email)) => write!(f, "{} <{}>", name, email),
            (None, Some(name), None) => write!(f, "{}", name),
            (None, None, Some(email)) => write!(f, "<{}>", email),
            (None, None, None) => write!(f, "↪Person"),
        }
    }
}

impl FromStr for PersonRef {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Person::from_str(input).map(Rc::new).map(PersonRef)
    }
}

#[cfg(feature = "tldr")]
impl tldr::Tldr for Person {
    type Error = core::fmt::Error;

    fn what(&self, ctx: &tldr::TldrContext) -> tldr::TldrResult<String, Self::Error> {
        use core::fmt::Write;
        use tldr::TldrLanguage::*;
        Ok(match ctx.language {
            English => {
                let mut tldr = String::new();
                write!(tldr, "A person named {}", self.name)?;
                Some(tldr)
            },
            _ => None,
        })
    }
}
