// This is free and unencumbered software released into the public domain.

use super::{literal::LangStrings, property::Property};
use std::collections::HashMap;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Class {
    pub id: String,
    #[cfg_attr(feature = "serde", serde(rename = "subclass_of"))]
    pub subclass_of: Option<String>,
    pub glyph: Option<String>,
    pub label: Option<LangStrings>,
    pub comment: Option<LangStrings>,
    #[cfg_attr(feature = "serde", serde(rename = "see_also"))]
    pub see_also: Option<LangStrings>,
    pub properties: Option<HashMap<String, Property>>,
}

impl Class {
    #[allow(unused)]
    pub fn new(
        id: String,
        subclass_of: Option<String>,
        glyph: Option<String>,
        label: Option<LangStrings>,
        comment: Option<LangStrings>,
        see_also: Option<LangStrings>,
        properties: Option<HashMap<String, Property>>,
    ) -> Self {
        Self {
            id,
            subclass_of,
            glyph,
            label,
            comment,
            see_also,
            properties,
        }
    }
}
