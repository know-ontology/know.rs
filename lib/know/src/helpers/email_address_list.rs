// This is free and unencumbered software released into the public domain.

use crate::datatypes::EmailAddress;

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EmailAddressList(Vec<EmailAddress>);

impl EmailAddressList {
    pub fn into_vec(self) -> Vec<EmailAddress> {
        self.0
    }
}

#[cfg(feature = "mail-parser")]
impl TryFrom<&mail_parser::Address<'_>> for EmailAddressList {
    type Error = ();

    fn try_from(input: &mail_parser::Address) -> Result<Self, Self::Error> {
        use mail_parser::Address::*;
        match input {
            List(addrs) => addrs.iter().try_fold(Self::default(), |mut output, addr| {
                output.0.push(EmailAddress::try_from(addr)?);
                Ok(output)
            }),
            Group(_groups) => todo!(), // TODO
        }
    }
}
