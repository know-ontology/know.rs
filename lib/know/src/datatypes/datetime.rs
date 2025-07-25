// This is free and unencumbered software released into the public domain.

use crate::{
    formatters::{DisplayInline, DisplayMime},
    traits,
};
use alloc::{fmt, str::FromStr};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateTime(jiff::Zoned);

impl DateTime {
    pub fn as_zoned(&self) -> &jiff::Zoned {
        &self.0
    }

    pub fn inline(&self) -> DisplayInline<DateTime> {
        DisplayInline(self)
    }

    pub fn mime(&self) -> DisplayMime<DateTime> {
        DisplayMime(self)
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_zoned().strftime("%FT%T.%3f%:z")) // ISO 8601
    }
}

impl fmt::Display for DisplayInline<'_, DateTime> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "📅 {}",
            self.0.as_zoned().strftime("%Y-%m-%d %H:%M:%S (%:z)")
        )
    }
}

impl fmt::Display for DisplayMime<'_, DateTime> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use jiff::fmt::{rfc2822::DateTimePrinter, StdFmtWrite};
        static P: DateTimePrinter = DateTimePrinter::new();
        P.print_zoned(&self.0.as_zoned(), StdFmtWrite(f))
            .map_err(|_| fmt::Error)
    }
}

impl FromStr for DateTime {
    type Err = jiff::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        jiff::Zoned::strptime("%FT%T.%3f%:z", input).map(|x| x.into()) // ISO 8601
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

#[cfg(feature = "mail-parser")]
impl TryFrom<&mail_parser::DateTime> for DateTime {
    type Error = jiff::Error;

    fn try_from(input: &mail_parser::DateTime) -> Result<Self, Self::Error> {
        let ts = jiff::Timestamp::from_second(input.to_timestamp())?;
        let off = jiff::tz::Offset::from_seconds(
            (input.tz_hour as i32 * 3_600 + input.tz_minute as i32 * 60)
                * if input.tz_before_gmt { 1 } else { -1 },
        )?;
        let tz = jiff::tz::TimeZone::fixed(off);
        Ok(Self(jiff::Zoned::new(ts, tz)))
    }
}

#[cfg(feature = "mail-parser")]
impl TryFrom<&mail_parser::HeaderValue<'_>> for DateTime {
    type Error = jiff::Error;

    fn try_from(input: &mail_parser::HeaderValue) -> Result<Self, Self::Error> {
        match input {
            mail_parser::HeaderValue::DateTime(value) => value.try_into(),
            _ => Err(jiff::Error::from_args(format_args!("not a date header"))), // unreachable?
        }
    }
}

#[cfg(feature = "mailparse")]
impl TryFrom<&mailparse::MailHeader<'_>> for DateTime {
    // `mailparse::MailParseError` if UTF-8 decoding fails
    // `jiff::Error` if RFC-2822 date parsing fails
    // See: https://github.com/BurntSushi/jiff/issues/262
    type Error = Box<dyn core::error::Error>;

    fn try_from(input: &mailparse::MailHeader) -> Result<Self, Self::Error> {
        let input_str = input.get_value_utf8()?;
        let input_date = jiff::fmt::rfc2822::parse(&input_str)?;
        Ok(Self(input_date))
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

#[cfg(feature = "serde")]
include!("datetime/serde.rs");
