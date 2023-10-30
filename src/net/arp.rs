use core::net::Ipv4Addr;

use super::{nic::MacAddress, Endianness, Serialise};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Arp {
    hardware_ty: [u8; 2],
    protocol_ty: [u8; 2],
    protocol_len: u8,
    hardware_len: u8,
    operand: u16,
    src_mac: MacAddress,
    src_ip: Ipv4Addr,
    dst_mac: MacAddress,
    dst_ip: Ipv4Addr,
}

impl Arp {
    pub const LEN: usize = core::mem::size_of::<Self>();
}

impl Serialise for Arp {
    fn deserialise(buffer: &[u8]) -> Self {
        let mut ptr = 0;

        Self {
            hardware_ty: consume!(ptr, buffer, [u8; 2]),
            protocol_ty: consume!(ptr, buffer, [u8; 2]),
            hardware_len: consume!(ptr, buffer, u8),
            protocol_len: consume!(ptr, buffer, u8),
            operand: consume!(ptr, buffer, Endianness::Big, u16),
            src_mac: consume!(ptr, buffer, [u8; 6]).into(),
            src_ip: consume!(ptr, buffer, [u8; 4]).into(),
            dst_mac: consume!(ptr, buffer, [u8; 6]).into(),
            dst_ip: consume!(ptr, buffer, [u8; 4]).into(),
        }
    }

    fn serialise(&self, buffer: &mut [u8]) {
        todo!()
    }
}
