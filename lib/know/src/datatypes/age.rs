// This is free and unencumbered software released into the public domain.

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
}

impl fmt::Display for Age {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "🎂 {}y", self.0)
    }
}

impl<T> From<T> for Age<T> {
    fn from(input: T) -> Self {
        Self(input)
    }
}
