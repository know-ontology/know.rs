// This is free and unencumbered software released into the public domain.

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct LangStrings {
    pub en: String,
}

impl LangStrings {
    pub fn new() -> Self {
        Self::default()
    }
}
