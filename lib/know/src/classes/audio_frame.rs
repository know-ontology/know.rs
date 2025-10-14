// This is free and unencumbered software released into the public domain.

use super::ThingLike;
use crate::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AudioFrame {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub id: Option<String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub rate: Option<usize>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub channels: Option<u8>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub samples: Option<usize>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub data: Vec<u8>,
}

pub trait AudioFrameLike: ThingLike {}

impl AudioFrameLike for AudioFrame {}

impl ThingLike for AudioFrame {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn name(&self) -> Option<&Name> {
        None
    }
}

#[cfg(feature = "serde")]
impl crate::traits::ToJsonLd for AudioFrame {
    fn to_jsonld(&self) -> serde_json::Result<serde_json::Value> {
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        use serde_json::json;
        Ok(json!({
            "@type": "AudioFrame",
            "@id": match self.id {
                Some(ref id) => id.clone(),
                None => "_:audio_frame".into(),
            },
            "rate": self.rate,
            "channels": self.channels,
            "samples": self.samples,
            "data": format!("data:audio/l16;base64,{}", STANDARD.encode(&self.data)),
        }))
    }
}
