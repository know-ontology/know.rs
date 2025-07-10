// This is free and unencumbered software released into the public domain.

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LangTag {
    #[cfg_attr(
        feature = "serde",
        serde(rename = "en", alias = "en-US", alias = "en-GB")
    )]
    #[default]
    English,
    Other(String),
}
