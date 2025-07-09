// This is free and unencumbered software released into the public domain.

use crate::formatters::DisplayInline;
use alloc::fmt;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Age<T = u8>(T);

impl<T> Age<T>
where
    T: Copy + Into<usize>,
{
    pub fn as_usize(&self) -> usize {
        self.0.into()
    }

    pub fn inline(&self) -> DisplayInline<Age<T>> {
        DisplayInline(self)
    }
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> fmt::Display for DisplayInline<'_, Age<T>>
where
    T: Copy + Into<usize>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "🎂 {}y", self.0.as_usize())
    }
}

impl<T> From<T> for Age<T> {
    fn from(input: T) -> Self {
        Self(input)
    }
}
