// This is free and unencumbered software released into the public domain.

use serde_json::{Result, Value};

pub trait ToJsonLd {
    fn to_jsonld(&self) -> Result<Value>;
}
