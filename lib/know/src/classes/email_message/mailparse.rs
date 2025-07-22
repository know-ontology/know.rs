// This is free and unencumbered software released into the public domain.

impl TryFrom<&mailparse::ParsedMail<'_>> for EmailMessage {
    type Error = mailparse::MailParseError;

    fn try_from(input: &mailparse::ParsedMail) -> Result<Self, Self::Error> {
        (&input.headers).try_into()
    }
}

impl TryFrom<&Vec<mailparse::MailHeader<'_>>> for EmailMessage {
    type Error = mailparse::MailParseError;

    fn try_from(input: &Vec<mailparse::MailHeader<'_>>) -> Result<Self, Self::Error> {
        use mailparse::{MailHeaderMap, MailParseError};
        Ok(Self {
            date: match input.get_first_header("Date") {
                None => Err(MailParseError::Generic("missing Date header"))?,
                Some(header) => header
                    .try_into()
                    .map_err(|_| MailParseError::Generic("invalid Date header"))?,
            },
            from: input
                .get_all_headers("From")
                .into_iter()
                .filter_map(|header| header.try_into().ok())
                .collect(),
            sender: input
                .get_first_value("Sender")
                .and_then(|header| header.parse().ok()),
            reply_to: input
                .get_all_headers("Reply-To")
                .into_iter()
                .filter_map(|header| header.try_into().ok())
                .collect(),
            to: input
                .get_all_headers("To")
                .into_iter()
                .filter_map(|header| header.try_into().ok())
                .collect(),
            cc: input
                .get_all_headers("Cc")
                .into_iter()
                .filter_map(|header| header.try_into().ok())
                .collect(),
            bcc: Default::default(),
            subject: input.get_first_value("Subject"),
            id: input
                .get_first_value("Message-ID")
                .and_then(|header| header.parse().ok()),
            in_reply_to: Default::default(), // TODO
            references: Default::default(),  // TODO
            body: Default::default(),
        })
    }
}
