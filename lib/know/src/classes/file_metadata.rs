// This is free and unencumbered software released into the public domain.

use super::ThingLike;

use crate::{
    datatypes::DateTime,
    formatters::{DisplayConcise, DisplayDetailed, DisplayInline, DisplayJsonLd, DisplayOneliner},
    prelude::*,
};
use alloc::fmt;

#[derive(Debug, Clone, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FileMetadata {
    #[cfg_attr(
        feature = "serde",
        serde(rename = "@id", skip_serializing_if = "Option::is_none")
    )]
    pub id: Option<String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub modification_date: Option<DateTime>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub size: Option<usize>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub owner: Option<String>,

    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub group: Option<String>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    pub filetype: FileType,
    //
    // TODO: figure out a cross-platform format
    // pub permissions: Option<Permissions>,
}

#[derive(Debug, Clone, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "filetype", rename_all = "lowercase"))]
pub enum FileType {
    #[default]
    Regular,
    Directory {
        #[cfg_attr(feature = "serde", serde(default))]
        children: Vec<String>,
    },
    Symlink {
        target: String,
    },
}

impl FileMetadata {
    pub fn inline(&self) -> DisplayInline<'_, Self> {
        DisplayInline(self)
    }

    pub fn oneliner(&self) -> DisplayOneliner<'_, Self> {
        DisplayOneliner(self)
    }

    pub fn concise(&self) -> DisplayConcise<'_, Self> {
        DisplayConcise(self)
    }

    pub fn detailed(&self) -> DisplayDetailed<'_, Self> {
        DisplayDetailed(self)
    }

    pub fn jsonld(&self) -> DisplayJsonLd<'_, Self> {
        DisplayJsonLd(self)
    }
}

impl fmt::Display for DisplayInline<'_, FileMetadata> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "📄 {}", self.0.id().unwrap_or("<unknown file>"))
    }
}

impl fmt::Display for DisplayOneliner<'_, FileMetadata> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "📄 {}", self.0.id().unwrap_or("<unknown file>"))?;
        let r#type = match &self.0.filetype {
            FileType::Regular => "regular",
            FileType::Directory { .. } => "directory",
            FileType::Symlink { .. } => "symlink",
        };
        write!(f, " is a {type} file")?;
        if let Some(ref size) = self.0.size {
            write!(f, " with size {size}")?;
        }
        if let Some(ref ts) = self.0.modification_date {
            write!(f, ", modified at {}", ts.inline())?;
        }
        Ok(())
    }
}

impl ThingLike for FileMetadata {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn name(&self) -> Option<&Name> {
        None
    }
}

#[cfg(feature = "serde")]
impl crate::traits::ToJsonLd for FileMetadata {
    fn to_jsonld(&self) -> serde_json::Result<serde_json::Value> {
        use serde_json::json;

        let mut value = json!({
            "@type": "FileMetadata",
            "@id": match self.id {
                Some(ref id) => id.clone(),
                None => "_:file-metadata".into(), // TODO: genid
            },
            "modification_date": self.modification_date,
            "size": self.size,
            "owner": self.owner,
            "group": self.group,
        });

        match &self.filetype {
            FileType::Regular => value["filetype"] = json!("regular"),
            FileType::Directory { children } => {
                value["filetype"] = json!("directory");
                value["children"] = json!(children);
            },
            FileType::Symlink { target } => {
                value["filetype"] = json!("symlink");
                value["target"] = json!(target);
            },
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "serde")]
    use crate::traits::ToJsonLd;

    #[cfg(feature = "serde")]
    #[test]
    fn test_to_jsonld_with_id() {
        let md = FileMetadata {
            id: Some("_:test-filemetadata-123".into()),
            ..Default::default()
        };

        let result = md.to_jsonld().unwrap();
        assert_eq!(result["@type"], "FileMetadata");
        assert_eq!(result["@id"], "_:test-filemetadata-123");
        assert_eq!(result["filetype"], "regular");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_to_jsonld_without_id() {
        let md = FileMetadata::default();

        let result = md.to_jsonld().unwrap();
        assert_eq!(result["@type"], "FileMetadata");
        assert_eq!(result["@id"], "_:file-metadata");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialization_roundtrip() {
        use serde_json;

        let now = jiff::Zoned::now().round(jiff::Unit::Millisecond).unwrap();
        let dt = DateTime::from(now);

        let original = FileMetadata {
            id: Some("_:roundtrip-test".to_string()),
            size: Some(42),
            owner: Some("file-owner".into()),
            group: Some("file-group".into()),
            modification_date: Some(dt.clone()),
            filetype: FileType::Regular,
        };

        // Serialize to JSON
        let json = serde_json::to_string(&original).unwrap();

        // Deserialize back
        let deserialized: FileMetadata = serde_json::from_str(&json).unwrap();

        // Should be equal
        assert_eq!(original, deserialized);

        let original = FileMetadata {
            id: Some("_:roundtrip-test".to_string()),
            size: Some(42),
            owner: Some("file-owner".into()),
            group: Some("file-group".into()),
            modification_date: Some(dt.clone()),
            filetype: FileType::Directory {
                children: vec!["one".into(), "two".into(), "three".into()],
            },
        };

        // Serialize to JSON
        let json = serde_json::to_string(&original).unwrap();

        // Deserialize back
        let deserialized: FileMetadata = serde_json::from_str(&json).unwrap();

        // Should be equal
        assert_eq!(original, deserialized);

        let original = FileMetadata {
            id: Some("_:roundtrip-test".to_string()),
            size: Some(42),
            owner: Some("file-owner".into()),
            group: Some("file-group".into()),
            modification_date: Some(dt),
            filetype: FileType::Symlink {
                target: "file:/path/to/target".into(),
            },
        };

        // Serialize to JSON
        let json = serde_json::to_string(&original).unwrap();

        // Deserialize back
        let deserialized: FileMetadata = serde_json::from_str(&json).unwrap();

        // Should be equal
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialization() {
        let json = serde_json::json!({
            "@type": "FileMetadata",
            "@id": "_:file-metadata",
            "size": 1,
            "filetype": "regular",
        });

        let file: FileMetadata = serde_json::from_value(json).unwrap();
        assert_eq!(file.id, Some("_:file-metadata".to_string()));
        assert_eq!(file.size, Some(1));
        assert_eq!(file.filetype, FileType::Regular);

        let json = serde_json::json!({
            "@type": "FileMetadata",
            "@id": "_:file-metadata",
            "modification_date": "2025-11-10T13:03:48.814+02:00",
            "size": 2,
            "owner": "file-owner",
            "group": "file-group",
            "filetype": "directory",
            "children": ["file:/path/to/child1", "file:/path/to/child2"],
        });

        let file: FileMetadata = serde_json::from_value(json).unwrap();
        assert_eq!(file.id, Some("_:file-metadata".to_string()));
        assert_eq!(file.size, Some(2));
        assert_eq!(
            file.filetype,
            FileType::Directory {
                children: vec!["file:/path/to/child1".into(), "file:/path/to/child2".into()]
            }
        );
        assert_eq!(file.owner.as_deref(), Some("file-owner"));
        assert_eq!(file.group.as_deref(), Some("file-group"));
        assert_eq!(
            file.modification_date.unwrap().0,
            jiff::Zoned::new(
                jiff::Timestamp::new(1762772628, 814000000).unwrap(),
                jiff::tz::TimeZone::fixed(jiff::tz::offset(2)),
            )
        );

        let json = serde_json::json!({
            "@type": "FileMetadata",
            "@id": "_:file-metadata",
            "size": 3,
            "filetype": "symlink",
            "target": "file:/path/to/target"
        });

        let file: FileMetadata = serde_json::from_value(json).unwrap();
        assert_eq!(file.id, Some("_:file-metadata".to_string()));
        assert_eq!(file.size, Some(3));
        assert_eq!(
            file.filetype,
            FileType::Symlink {
                target: "file:/path/to/target".into()
            }
        );
    }
}
