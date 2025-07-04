// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
    nla::{DefaultNla, Nla, NlaBuffer},
    DecodeError, Parseable,
};

#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum InfoGreTap6 {
    Other(DefaultNla),
}

impl Nla for InfoGreTap6 {
    fn value_len(&self) -> usize {
        match self {
            Self::Other(nla) => nla.value_len(),
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        match self {
            Self::Other(nla) => nla.emit_value(buffer),
        }
    }

    fn kind(&self) -> u16 {
        match self {
            Self::Other(nla) => nla.kind(),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for InfoGreTap6 {
    type Error = DecodeError;
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        #[allow(clippy::match_single_binding)]
        Ok(match buf.kind() {
            _kind => Self::Other(DefaultNla::parse(buf)?),
        })
    }
}
