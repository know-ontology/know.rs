// This is free and unencumbered software released into the public domain.

impl traits::ToJsonLd for EmailAddress {
    fn to_jsonld(&self) -> serde_json::Result<serde_json::Value> {
        Ok(format!("mailto:{}", self.0).into())
    }
}
