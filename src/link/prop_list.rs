// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
    nla::{DefaultNla, Nla, NlaBuffer},
    parsers::parse_string,
    traits::Parseable,
    DecodeError,
};

const IFLA_ALT_IFNAME: u16 = 53;

#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum Prop {
    AltIfName(String),
    Other(DefaultNla),
}

impl Nla for Prop {
    #[rustfmt::skip]
    fn value_len(&self) -> usize {
        use self::Prop::*;
        match self {
            AltIfName(ref string) => string.len() + 1,
            Other(nla) => nla.value_len()
        }
    }

    #[rustfmt::skip]
    fn emit_value(&self, buffer: &mut [u8]) {
        use self::Prop::*;
        match self {
            AltIfName(ref string) => {
                buffer[..string.len()].copy_from_slice(string.as_bytes());
                buffer[string.len()] = 0;
            },
            Other(nla) => nla.emit_value(buffer)
        }
    }

    fn kind(&self) -> u16 {
        use self::Prop::*;
        match self {
            AltIfName(_) => IFLA_ALT_IFNAME,
            Other(nla) => nla.kind(),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for Prop {
    type Error = DecodeError;
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            IFLA_ALT_IFNAME => Prop::AltIfName(parse_string(payload)?),
            _kind => Prop::Other(DefaultNla::parse(buf)?),
        })
    }
}
