// This is free and unencumbered software released into the public domain.

use super::ThingLike;
use crate::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Image {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub id: Option<String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub width: Option<usize>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub height: Option<usize>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub data: Vec<u8>,
}

pub trait ImageLike: ThingLike {}

impl ImageLike for Image {}

impl ThingLike for Image {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn name(&self) -> Option<&Name> {
        None
    }
}

#[cfg(feature = "serde")]
impl crate::traits::ToJsonLd for Image {
    fn to_jsonld(&self) -> serde_json::Result<serde_json::Value> {
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        use serde_json::json;
        Ok(json!({
            "@type": "Image",
            "@id": match self.id {
                Some(ref id) => id.clone(),
                None => "_:image".into(),
            },
            "width": self.width,
            "height": self.height,
            "data": format!("data:image/rgb;base64,{}", STANDARD.encode(&self.data)),
        }))
    }
}
