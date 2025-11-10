// This is free and unencumbered software released into the public domain.

use super::ThingLike;
use crate::prelude::*;

pub trait FileLike: ThingLike {
    fn size(&self) -> u64;
}

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct File {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<Name>,

    #[cfg_attr(
        feature = "serde",
        serde(rename = "@id", skip_serializing_if = "Option::is_none")
    )]
    pub id: Option<String>,

    pub size: u64,

    pub content_type: Option<String>,

    #[cfg_attr(
        feature = "serde",
        serde(
            skip_serializing_if = "FileData::is_empty",
            serialize_with = "serialize_data",
            deserialize_with = "deserialize_data"
        )
    )]
    pub data: FileData,
}

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct FileData {
    pub r#type: Option<String>,
    pub data: Vec<u8>,
}

impl FileData {
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl AsRef<FileData> for FileData {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ThingLike for File {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn name(&self) -> Option<&Name> {
        self.name.as_ref()
    }
}

impl FileLike for File {
    fn size(&self) -> u64 {
        self.size
    }
}

#[cfg(feature = "serde")]
impl crate::traits::ToJsonLd for File {
    fn to_jsonld(&self) -> serde_json::Result<serde_json::Value> {
        let data = if self.data.is_empty() {
            None
        } else {
            Some(serialize_data(&self.data, serde_json::value::Serializer)?)
        };
        Ok(serde_json::json!({
            "@type": "File",
            "@id": match self.id {
                Some(ref id) => id.clone(),
                None => "_:file".into(), // TODO: genid
            },
            "data": data,
        }))
    }
}

#[cfg(feature = "serde")]
fn serialize_data<T, S>(data: T, ser: S) -> std::result::Result<S::Ok, S::Error>
where
    T: AsRef<FileData>,
    S: serde::Serializer,
{
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use serde::Serialize;
    let data = data.as_ref();

    let data = if let Some(ref content_type) = data.r#type {
        use base64::prelude::{BASE64_STANDARD, Engine as _};
        let encoded = BASE64_STANDARD.encode(&data.data);
        format!("data:{};base64,{}", content_type, encoded)
    } else if let Ok(content) = std::str::from_utf8(&data.data) {
        let content_type = "text/plain;charset=utf-8";
        format!("data:{},{}", content_type, content)
    } else {
        let content_type = "application/octet-stream";
        use base64::prelude::{BASE64_STANDARD, Engine as _};
        let encoded = BASE64_STANDARD.encode(&data.data);
        format!("data:{};base64,{}", content_type, encoded)
    };

    data.serialize(ser)
}

#[cfg(feature = "serde")]
fn deserialize_data<'de, D>(deserializer: D) -> std::result::Result<FileData, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    let data_url = String::deserialize(deserializer)?;

    let (content_type, data) = data_url
        .strip_prefix("data:")
        .unwrap()
        .split_once(",")
        .unwrap();

    let (content_type, is_base64) = if let Some(content_type) = content_type.strip_suffix(";base64")
    {
        (content_type, true)
    } else {
        (content_type, false)
    };

    let data = if is_base64 {
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        STANDARD.decode(data).map_err(serde::de::Error::custom)?
    } else {
        data.into()
    };

    Ok(FileData {
        r#type: (!content_type.is_empty()).then(|| content_type.into()),
        data,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "serde")]
    use crate::traits::ToJsonLd;

    #[test]
    fn test_file_default() {
        let file = File::default();
        assert_eq!(file.id, None);
        assert_eq!(file.name, None);
        assert_eq!(file.size, 0);
        assert!(file.data.is_empty());
    }

    #[test]
    fn test_thing_like_trait() {
        let file_with_id = File {
            id: Some("_:test-id".to_string()),
            ..Default::default()
        };

        let file_without_id = File::default();

        assert_eq!(file_with_id.id(), Some("_:test-id"));
        assert_eq!(file_without_id.id(), None);
        assert_eq!(file_with_id.name(), None);
        assert_eq!(file_without_id.name(), None);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_to_jsonld_with_id() {
        let file = File {
            id: Some("_:test-file-123".to_string()),
            data: FileData {
                r#type: None,
                data: vec![255, 0, 0],
            },
            ..Default::default()
        };

        let result = file.to_jsonld().unwrap();
        assert_eq!(result["@type"], "File");
        assert_eq!(result["@id"], "_:test-file-123");

        let data_str = result["data"].as_str().unwrap();
        assert!(data_str.starts_with("data:application/octet-stream;base64,"));
        assert!(data_str.contains("/wAA")); // Base64 encoded [255, 0, 0]
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_to_jsonld_without_id() {
        let file = File::default();

        let result = file.to_jsonld().unwrap();
        assert_eq!(result["@type"], "File");
        assert_eq!(result["@id"], "_:file");
        assert_eq!(result["data"].as_str(), None);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization_roundtrip() {
        use serde_json;

        let original = File {
            id: Some("_:roundtrip-test".to_string()),
            data: FileData {
                r#type: Some("application/octet-stream".into()),

                data: vec![128, 64, 32, 16, 8, 4, 2, 1],
            },
            ..Default::default()
        };

        // Serialize to JSON
        let json = serde_json::to_string(&original).unwrap();

        // Deserialize back
        let deserialized: File = serde_json::from_str(&json).unwrap();

        // Should be equal
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization_skips_empty_fields() {
        use serde_json::{self, Value};

        let file = File::default();

        let json_value: Value = serde_json::to_value(&file).unwrap();
        let json_obj = json_value.as_object().unwrap();

        // These fields should be skipped in serialization
        assert!(!json_obj.contains_key("@id"));
        assert!(!json_obj.contains_key("width"));
        assert!(!json_obj.contains_key("height"));
        assert!(!json_obj.contains_key("data")); // Empty vec should be skipped
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization_includes_populated_fields() {
        use serde_json::{self, Value};

        let file = File {
            id: Some("_:populated".to_string()),
            data: FileData {
                r#type: None,
                data: vec![1, 2, 3],
            },
            ..Default::default()
        };

        let json_value: Value = serde_json::to_value(&file).unwrap();
        let json_obj = json_value.as_object().unwrap();

        // These fields should be included
        assert!(json_obj.contains_key("@id"));
        assert!(json_obj.contains_key("data"));

        assert_eq!(json_obj["@id"], "_:populated");
        assert_eq!(
            json_obj["data"],
            serde_json::json!("data:text/plain;charset=utf-8,\u{1}\u{2}\u{3}")
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialization() {
        let json = serde_json::json!({
            "@type": "File",
            "@id": "_:file",
            "size": 3,
            "data": "data:application/octet-stream;base64,AQID",
        });

        let file: File = serde_json::from_value(json).unwrap();
        assert_eq!(file.id, Some("_:file".to_string()));
        assert_eq!(file.data.data, vec![1, 2, 3]);
    }
}
