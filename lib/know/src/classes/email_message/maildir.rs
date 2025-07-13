// This is free and unencumbered software released into the public domain.

impl TryFrom<&mut maildir::MailEntry> for EmailMessage {
    type Error = maildir::MailEntryError;

    fn try_from(input: &mut maildir::MailEntry) -> Result<Self, Self::Error> {
        Ok((&input.headers()?).try_into()?)
    }
}
