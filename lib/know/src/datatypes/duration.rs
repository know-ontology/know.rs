// This is free and unencumbered software released into the public domain.

use crate::{
    formatters::{DisplayInline, DisplayMime},
    traits,
};
use alloc::{fmt, str::FromStr};

#[derive(Clone, Debug, Default)]
pub struct Duration(pub(crate) jiff::Span);

impl Duration {
    pub fn zero() -> Self {
        Self(jiff::Span::new())
    }

    pub fn round(&self) -> Self {
        self.clone() // TODO
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#}", self.0)
    }
}

impl FromStr for Duration {
    type Err = jiff::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(input.parse()?))
    }
}

impl From<jiff::Span> for Duration {
    fn from(input: jiff::Span) -> Self {
        Self(input)
    }
}

impl From<&jiff::Span> for Duration {
    fn from(input: &jiff::Span) -> Self {
        Self(input.clone())
    }
}

impl Into<jiff::Span> for Duration {
    fn into(self) -> jiff::Span {
        self.0
    }
}

impl AsRef<jiff::Span> for Duration {
    fn as_ref(&self) -> &jiff::Span {
        &self.0
    }
}
