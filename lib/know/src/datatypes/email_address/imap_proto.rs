// This is free and unencumbered software released into the public domain.

impl TryFrom<&imap_proto::Address<'_>> for EmailAddress {
    type Error = ();

    fn try_from(input: &imap_proto::Address) -> Result<Self, Self::Error> {
        match (&input.mailbox, &input.host) {
            (Some(mailbox), Some(host)) => {
                let mailbox = String::from_utf8_lossy(&mailbox);
                let host = String::from_utf8_lossy(&host);
                Ok(Self(format!("{}@{}", mailbox, host).to_lowercase()))
            },
            _ => Err(()),
        }
    }
}
