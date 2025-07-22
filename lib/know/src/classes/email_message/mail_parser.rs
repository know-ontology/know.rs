// This is free and unencumbered software released into the public domain.

impl TryFrom<&mail_parser::Message<'_>> for EmailMessage {
    type Error = ();

    fn try_from(input: &mail_parser::Message) -> Result<Self, Self::Error> {
        use crate::helpers::EmailAddressList;
        Ok(Self {
            date: input
                .date()
                .expect("message must have a Date header") // FIXME
                .try_into()
                .unwrap(),
            from: input
                .from()
                .and_then(|addresses| EmailAddressList::try_from(addresses).ok())
                .map(EmailAddressList::into_vec)
                .unwrap_or_default(),
            sender: Default::default(),   // TODO
            reply_to: Default::default(), // TODO
            to: input
                .to()
                .and_then(|addresses| EmailAddressList::try_from(addresses).ok())
                .map(EmailAddressList::into_vec)
                .unwrap_or_default(),
            cc: input
                .cc()
                .and_then(|addresses| EmailAddressList::try_from(addresses).ok())
                .map(EmailAddressList::into_vec)
                .unwrap_or_default(),
            bcc: Default::default(),
            subject: input.subject().map(ToString::to_string),
            id: input.message_id().map(EmailMessageId::from),
            in_reply_to: Default::default(), // TODO
            references: Default::default(),  // TODO
            body: input.body_text(0).map(|s| s.into_owned()),
        })
    }
}
