// This is free and unencumbered software released into the public domain.

impl TryFrom<&mail_parser::Addr<'_>> for EmailAddress {
    type Error = ();

    fn try_from(input: &mail_parser::Addr) -> Result<Self, Self::Error> {
        Ok(Self(
            input
                .address
                .as_ref()
                .map(|s| s.to_lowercase())
                .ok_or_else(|| ())?,
        ))
    }
}
