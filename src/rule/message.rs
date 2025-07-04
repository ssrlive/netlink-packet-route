// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
    traits::{Emitable, Parseable},
    DecodeError,
};

use super::{RuleAttribute, RuleHeader, RuleMessageBuffer};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
pub struct RuleMessage {
    pub header: RuleHeader,
    pub attributes: Vec<RuleAttribute>,
}

impl Emitable for RuleMessage {
    fn buffer_len(&self) -> usize {
        self.header.buffer_len() + self.attributes.as_slice().buffer_len()
    }

    fn emit(&self, buffer: &mut [u8]) {
        self.header.emit(buffer);
        self.attributes
            .as_slice()
            .emit(&mut buffer[self.header.buffer_len()..]);
    }
}

impl<'a, T: AsRef<[u8]> + 'a> Parseable<RuleMessageBuffer<&'a T>>
    for RuleMessage
{
    type Error = DecodeError;
    fn parse(buf: &RuleMessageBuffer<&'a T>) -> Result<Self, DecodeError> {
        let header = RuleHeader::parse(buf)?;
        let attributes = Vec::<RuleAttribute>::parse(buf)?;
        Ok(RuleMessage { header, attributes })
    }
}

impl<'a, T: AsRef<[u8]> + 'a> Parseable<RuleMessageBuffer<&'a T>>
    for Vec<RuleAttribute>
{
    type Error = DecodeError;
    fn parse(buf: &RuleMessageBuffer<&'a T>) -> Result<Self, DecodeError> {
        let mut attributes = vec![];
        for nla_buf in buf.attributes() {
            attributes.push(RuleAttribute::parse(&nla_buf?)?);
        }
        Ok(attributes)
    }
}
