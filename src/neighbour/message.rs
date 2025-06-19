// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
    traits::{Emitable, Parseable, ParseableParametrized},
    DecodeError,
};

use super::{
    super::AddressFamily, NeighbourAttribute, NeighbourHeader,
    NeighbourMessageBuffer,
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
pub struct NeighbourMessage {
    pub header: NeighbourHeader,
    pub attributes: Vec<NeighbourAttribute>,
}

impl Emitable for NeighbourMessage {
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

impl<'a, T: AsRef<[u8]> + 'a> Parseable<NeighbourMessageBuffer<&'a T>>
    for NeighbourMessage
{
    type Error = DecodeError;
    fn parse(buf: &NeighbourMessageBuffer<&'a T>) -> Result<Self, DecodeError> {
        let header = NeighbourHeader::parse(buf)?;
        let address_family = header.family;
        Ok(NeighbourMessage {
            header,
            attributes: Vec::<NeighbourAttribute>::parse_with_param(
                buf,
                address_family,
            )?,
        })
    }
}

impl<'a, T: AsRef<[u8]> + 'a>
    ParseableParametrized<NeighbourMessageBuffer<&'a T>, AddressFamily>
    for Vec<NeighbourAttribute>
{
    type Error = DecodeError;
    fn parse_with_param(
        buf: &NeighbourMessageBuffer<&'a T>,
        address_family: AddressFamily,
    ) -> Result<Self, DecodeError> {
        let mut attributes = vec![];
        for nla_buf in buf.attributes() {
            attributes.push(NeighbourAttribute::parse_with_param(
                &nla_buf?,
                address_family,
            )?);
        }
        Ok(attributes)
    }
}
