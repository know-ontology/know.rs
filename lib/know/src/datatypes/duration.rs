// This is free and unencumbered software released into the public domain.

use alloc::{fmt, str::FromStr};

#[derive(Clone, Debug, Default)]
pub struct Duration(pub(crate) jiff::Span);

impl Duration {
    pub fn zero() -> Self {
        Self(jiff::Span::new())
    }

    /// Round the duration for display to humans with a "just do the right
    /// thing" philosophy.
    pub fn round(&self) -> Result<Self, jiff::Error> {
        use jiff::{RoundMode, Span, SpanRound, Unit};
        let span = &self.0.round(
            SpanRound::new()
                .largest(Unit::Year)
                .smallest(Unit::Second)
                .mode(RoundMode::Expand)
                .relative(&jiff::Zoned::now()),
        )?;
        let span = if span.get_years() >= 10 {
            Span::new().years(span.get_years())
        } else if span.get_years() > 0 {
            Span::new()
                .years(span.get_years())
                .months(span.get_months())
        } else if span.get_months() > 0 {
            Span::new().months(span.get_months()).days(span.get_days())
        } else if span.get_days() > 0 {
            Span::new().days(span.get_days()).hours(span.get_hours())
        } else if span.get_hours() > 0 {
            Span::new()
                .hours(span.get_hours())
                .minutes(span.get_minutes())
        } else if span.get_minutes() > 0 {
            Span::new()
                .minutes(span.get_minutes())
                .seconds(span.get_seconds())
        } else if span.get_seconds() > 0 {
            Span::new().seconds(span.get_seconds())
        } else {
            Span::new()
        };
        Ok(Self(span))
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
