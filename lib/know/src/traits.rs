// This is free and unencumbered software released into the public domain.

#[cfg(feature = "serde")]
mod jsonld;
#[cfg(feature = "serde")]
pub use jsonld::*;
