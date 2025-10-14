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

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "serde")]
    use crate::traits::ToJsonLd;

    #[test]
    fn test_image_default() {
        let image = Image::default();
        assert_eq!(image.id, None);
        assert_eq!(image.width, None);
        assert_eq!(image.height, None);
        assert!(image.data.is_empty());
    }

    #[test]
    fn test_image_new_with_fields() {
        let image = Image {
            id: Some("test-image".to_string()),
            width: Some(800),
            height: Some(600),
            data: vec![255, 0, 0, 0, 255, 0, 0, 0, 255], // RGB pixel data
        };

        assert_eq!(image.id, Some("test-image".to_string()));
        assert_eq!(image.width, Some(800));
        assert_eq!(image.height, Some(600));
        assert_eq!(image.data, vec![255, 0, 0, 0, 255, 0, 0, 0, 255]);
    }

    #[test]
    fn test_image_clone() {
        let original = Image {
            id: Some("original".to_string()),
            width: Some(100),
            height: Some(100),
            data: vec![1, 2, 3, 4, 5],
        };

        let cloned = original.clone();
        assert_eq!(original, cloned);
        assert_eq!(cloned.id, Some("original".to_string()));
        assert_eq!(cloned.width, Some(100));
        assert_eq!(cloned.height, Some(100));
        assert_eq!(cloned.data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_image_equality() {
        let image1 = Image {
            id: Some("test".to_string()),
            width: Some(50),
            height: Some(50),
            data: vec![1, 2, 3],
        };

        let image2 = Image {
            id: Some("test".to_string()),
            width: Some(50),
            height: Some(50),
            data: vec![1, 2, 3],
        };

        let image3 = Image {
            id: Some("different".to_string()),
            width: Some(50),
            height: Some(50),
            data: vec![1, 2, 3],
        };

        assert_eq!(image1, image2);
        assert_ne!(image1, image3);
    }

    #[test]
    fn test_image_ordering() {
        let image1 = Image {
            id: Some("a".to_string()),
            width: Some(10),
            height: Some(10),
            data: vec![1],
        };

        let image2 = Image {
            id: Some("b".to_string()),
            width: Some(10),
            height: Some(10),
            data: vec![1],
        };

        assert!(image1 < image2);
        assert!(image2 > image1);
    }

    #[test]
    fn test_image_debug() {
        let image = Image {
            id: Some("debug-test".to_string()),
            width: Some(32),
            height: Some(32),
            data: vec![255, 128, 0],
        };

        let debug_str = format!("{:?}", image);
        assert!(debug_str.contains("debug-test"));
        assert!(debug_str.contains("32"));
        assert!(debug_str.contains("255"));
    }

    #[test]
    fn test_thing_like_trait() {
        let image_with_id = Image {
            id: Some("test-id".to_string()),
            width: None,
            height: None,
            data: vec![],
        };

        let image_without_id = Image::default();

        assert_eq!(image_with_id.id(), Some("test-id"));
        assert_eq!(image_without_id.id(), None);
        assert_eq!(image_with_id.name(), None);
        assert_eq!(image_without_id.name(), None);
    }

    #[test]
    fn test_image_like_trait() {
        let image = Image::default();
        // Test that Image implements ImageLike trait
        fn test_image_like<T: ImageLike>(_: T) {}
        test_image_like(image);
    }

    #[test]
    fn test_image_with_large_data() {
        let large_data = vec![42; 1000000]; // 1MB of data
        let image = Image {
            id: Some("large-image".to_string()),
            width: Some(1000),
            height: Some(1000),
            data: large_data.clone(),
        };

        assert_eq!(image.data.len(), 1000000);
        assert_eq!(image.data[0], 42);
        assert_eq!(image.data[999999], 42);
    }

    #[test]
    fn test_image_partial_fields() {
        let image_only_width = Image {
            id: None,
            width: Some(640),
            height: None,
            data: vec![],
        };

        let image_only_height = Image {
            id: None,
            width: None,
            height: Some(480),
            data: vec![],
        };

        assert_eq!(image_only_width.width, Some(640));
        assert_eq!(image_only_width.height, None);
        assert_eq!(image_only_height.width, None);
        assert_eq!(image_only_height.height, Some(480));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_to_jsonld_with_id() {
        let image = Image {
            id: Some("test-image-123".to_string()),
            width: Some(800),
            height: Some(600),
            data: vec![255, 0, 0], // Simple RGB data
        };

        let result = image.to_jsonld().unwrap();
        assert_eq!(result["@type"], "Image");
        assert_eq!(result["@id"], "test-image-123");
        assert_eq!(result["width"], 800);
        assert_eq!(result["height"], 600);

        let data_str = result["data"].as_str().unwrap();
        assert!(data_str.starts_with("data:image/rgb;base64,"));
        assert!(data_str.contains("/wAA")); // Base64 encoded [255, 0, 0]
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_to_jsonld_without_id() {
        let image = Image {
            id: None,
            width: Some(100),
            height: Some(100),
            data: vec![0, 255, 0], // Green pixel
        };

        let result = image.to_jsonld().unwrap();
        assert_eq!(result["@type"], "Image");
        assert_eq!(result["@id"], "_:image");
        assert_eq!(result["width"], 100);
        assert_eq!(result["height"], 100);

        let data_str = result["data"].as_str().unwrap();
        assert!(data_str.starts_with("data:image/rgb;base64,"));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_to_jsonld_empty_data() {
        let image = Image {
            id: Some("empty-image".to_string()),
            width: Some(0),
            height: Some(0),
            data: vec![],
        };

        let result = image.to_jsonld().unwrap();
        assert_eq!(result["@type"], "Image");
        assert_eq!(result["@id"], "empty-image");
        assert_eq!(result["width"], 0);
        assert_eq!(result["height"], 0);
        assert_eq!(result["data"], "data:image/rgb;base64,");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization_roundtrip() {
        use serde_json;

        let original = Image {
            id: Some("roundtrip-test".to_string()),
            width: Some(256),
            height: Some(256),
            data: vec![128, 64, 32, 16, 8, 4, 2, 1],
        };

        // Serialize to JSON
        let json = serde_json::to_string(&original).unwrap();

        // Deserialize back
        let deserialized: Image = serde_json::from_str(&json).unwrap();

        // Should be equal
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization_skips_empty_fields() {
        use serde_json::{self, Value};

        let image = Image {
            id: None,
            width: None,
            height: None,
            data: vec![],
        };

        let json_value: Value = serde_json::to_value(&image).unwrap();
        let json_obj = json_value.as_object().unwrap();

        // These fields should be skipped in serialization
        assert!(!json_obj.contains_key("id"));
        assert!(!json_obj.contains_key("width"));
        assert!(!json_obj.contains_key("height"));
        assert!(!json_obj.contains_key("data")); // Empty vec should be skipped
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization_includes_populated_fields() {
        use serde_json::{self, Value};

        let image = Image {
            id: Some("populated".to_string()),
            width: Some(42),
            height: Some(24),
            data: vec![1, 2, 3],
        };

        let json_value: Value = serde_json::to_value(&image).unwrap();
        let json_obj = json_value.as_object().unwrap();

        // These fields should be included
        assert!(json_obj.contains_key("id"));
        assert!(json_obj.contains_key("width"));
        assert!(json_obj.contains_key("height"));
        assert!(json_obj.contains_key("data"));

        assert_eq!(json_obj["id"], "populated");
        assert_eq!(json_obj["width"], 42);
        assert_eq!(json_obj["height"], 24);
        assert_eq!(json_obj["data"], serde_json::json!([1, 2, 3]));
    }
}
