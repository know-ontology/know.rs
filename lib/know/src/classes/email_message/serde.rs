// This is free and unencumbered software released into the public domain.

impl fmt::Display for DisplayJsonLd<'_, EmailMessage> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::traits::ToJsonLd;
        let json = self.0.to_jsonld().unwrap(); // infallible
        if cfg!(feature = "pretty") {
            let mut w = crate::formatters::WriteToFormatter::new(f);
            colored_json::write_colored_json(&json, &mut w).map_err(|_| fmt::Error)?;
            writeln!(f)
        } else {
            writeln!(f, "{}", json)
        }
    }
}

impl traits::ToJsonLd for EmailMessage {
    fn to_jsonld(&self) -> serde_json::Result<serde_json::Value> {
        use serde_json::json;
        Ok(json!({
            "@id": match self.id {
                Some(ref id) => id.to_jsonld()?,
                None => "_:message".into(),
            },
            "@type": "EmailMessage",
            "from": self.from.iter().filter_map(|x| x.to_jsonld().ok()).collect::<Vec<_>>(),
            "sender": self.sender.iter().filter_map(|x| x.to_jsonld().ok()).collect::<Vec<_>>(),
            "to": self.to.iter().filter_map(|x| x.to_jsonld().ok()).collect::<Vec<_>>(),
            "cc": self.cc.iter().filter_map(|x| x.to_jsonld().ok()).collect::<Vec<_>>(),
            "bcc": self.bcc.iter().filter_map(|x| x.to_jsonld().ok()).collect::<Vec<_>>(),
            "subject": self.subject,
            "body": self.body,
        }))
    }
}
