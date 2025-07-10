// This is free and unencumbered software released into the public domain.

use super::literal::LangStrings;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Property {
    pub id: Option<String>,
    pub range: Option<String>,
    pub label: Option<LangStrings>,
    pub comment: Option<LangStrings>,
    pub see_also: Option<LangStrings>,
}

impl Property {
    #[allow(unused)]
    pub fn new(
        id: Option<String>,
        range: Option<String>,
        label: Option<LangStrings>,
        comment: Option<LangStrings>,
        see_also: Option<LangStrings>,
    ) -> Self {
        Self {
            id,
            range,
            label,
            comment,
            see_also,
        }
    }
}
