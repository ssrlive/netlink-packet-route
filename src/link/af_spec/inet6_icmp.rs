// SPDX-License-Identifier: MIT

use netlink_packet_utils::{
    traits::{Emitable, Parseable},
    DecodeError,
};

pub(crate) const ICMP6_STATS_LEN: usize = 56;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
#[non_exhaustive]
pub struct Icmp6Stats {
    pub num: i64,
    pub in_msgs: i64,
    pub in_errors: i64,
    pub out_msgs: i64,
    pub out_errors: i64,
    pub csum_errors: i64,
    pub rate_limit_host: i64,
}

buffer!(Icmp6StatsBuffer(ICMP6_STATS_LEN) {
    num: (i64, 0..8),
    in_msgs: (i64, 8..16),
    in_errors: (i64, 16..24),
    out_msgs: (i64, 24..32),
    out_errors: (i64, 32..40),
    csum_errors: (i64, 40..48),
    rate_limit_host: (i64, 48..56),
});

impl<T: AsRef<[u8]>> Parseable<Icmp6StatsBuffer<T>> for Icmp6Stats {
    type Error = DecodeError;
    fn parse(buf: &Icmp6StatsBuffer<T>) -> Result<Self, DecodeError> {
        Ok(Self {
            num: buf.num(),
            in_msgs: buf.in_msgs(),
            in_errors: buf.in_errors(),
            out_msgs: buf.out_msgs(),
            out_errors: buf.out_errors(),
            csum_errors: buf.csum_errors(),
            rate_limit_host: buf.rate_limit_host(),
        })
    }
}

impl Emitable for Icmp6Stats {
    fn buffer_len(&self) -> usize {
        ICMP6_STATS_LEN
    }

    fn emit(&self, buffer: &mut [u8]) {
        let mut buffer = Icmp6StatsBuffer::new(buffer);
        buffer.set_num(self.num);
        buffer.set_in_msgs(self.in_msgs);
        buffer.set_in_errors(self.in_errors);
        buffer.set_out_msgs(self.out_msgs);
        buffer.set_out_errors(self.out_errors);
        buffer.set_csum_errors(self.csum_errors);
        buffer.set_rate_limit_host(self.rate_limit_host);
    }
}
