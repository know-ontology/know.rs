// This is free and unencumbered software released into the public domain.

impl TryFrom<&imap_proto::types::Envelope<'_>> for EmailMessage {
    type Error = ();

    fn try_from(input: &imap_proto::types::Envelope) -> Result<Self, Self::Error> {
        let input_date = String::from_utf8_lossy(input.date.as_ref().unwrap()); // TODO: fallible?
        let input_date = jiff::fmt::rfc2822::parse(&input_date).map_err(|_| ())?;
        Ok(Self {
            date: input_date.into(),
            from: input
                .from
                .as_ref()
                .map(|xs| xs.into_iter().filter_map(|x| x.try_into().ok()).collect())
                .unwrap_or_default(),
            sender: input
                .sender
                .as_ref()
                .map(|xs| xs.into_iter().filter_map(|x| x.try_into().ok()).next())
                .unwrap_or_default(),
            reply_to: input
                .reply_to
                .as_ref()
                .map(|xs| xs.into_iter().filter_map(|x| x.try_into().ok()).collect())
                .unwrap_or_default(),
            to: input
                .to
                .as_ref()
                .map(|xs| xs.into_iter().filter_map(|x| x.try_into().ok()).collect())
                .unwrap_or_default(),
            cc: input
                .cc
                .as_ref()
                .map(|xs| xs.into_iter().filter_map(|x| x.try_into().ok()).collect())
                .unwrap_or_default(),
            bcc: Default::default(),
            subject: input
                .subject
                .as_ref()
                .map(|x| String::from_utf8_lossy(x).into_owned()),
            id: input
                .message_id
                .as_ref()
                .and_then(|header| String::from_utf8_lossy(header).parse().ok()),
            in_reply_to: Default::default(), // TODO
            references: Default::default(),  // TODO
            body: Default::default(),
        })
    }
}
