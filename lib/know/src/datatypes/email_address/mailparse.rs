// This is free and unencumbered software released into the public domain.

impl TryFrom<&mailparse::MailAddr> for EmailAddress {
    type Error = mailparse::MailParseError;

    fn try_from(input: &mailparse::MailAddr) -> Result<Self, Self::Error> {
        match input {
            mailparse::MailAddr::Single(single) => Ok(Self(single.addr.to_lowercase())),
            mailparse::MailAddr::Group(_group) => todo!(),
        }
    }
}

impl TryFrom<&mailparse::MailHeader<'_>> for EmailAddress {
    type Error = mailparse::MailParseError;

    fn try_from(input: &mailparse::MailHeader) -> Result<Self, Self::Error> {
        use mailparse::MailParseError;
        (input.get_value_utf8()?)
            .parse()
            .map_err(|_| MailParseError::Generic("invalid email address in header"))
    }
}
