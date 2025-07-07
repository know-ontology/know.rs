// This is free and unencumbered software released into the public domain.

use alloc::fmt;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DateTime(jiff::Zoned);

impl DateTime {
    pub fn as_zoned(&self) -> &jiff::Zoned {
        &self.0
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "📅 {}", self.0)
    }
}

impl From<i64> for DateTime {
    fn from(input: i64) -> Self {
        jiff::Timestamp::new(input, 0).unwrap().into()
    }
}

impl From<jiff::Timestamp> for DateTime {
    fn from(input: jiff::Timestamp) -> Self {
        Self(jiff::Zoned::new(input, jiff::tz::TimeZone::UTC))
    }
}

impl From<jiff::Zoned> for DateTime {
    fn from(input: jiff::Zoned) -> Self {
        Self(input)
    }
}

impl From<&jiff::Zoned> for DateTime {
    fn from(input: &jiff::Zoned) -> Self {
        Self(input.clone())
    }
}

impl Into<jiff::Zoned> for DateTime {
    fn into(self) -> jiff::Zoned {
        self.0
    }
}

impl AsRef<jiff::Zoned> for DateTime {
    fn as_ref(&self) -> &jiff::Zoned {
        &self.0
    }
}
