use super::{arp::Arp, nic::MacAddress, Serialise};

#[derive(Debug)]
enum EtherType {
    /// 0x0800
    IPv4,

    /// 0x0806
    Arp,

    /// 0x86DD
    IPv6,

    /// 0x88E1
    HomePlugAV,

    Unknown([u8; 2]),
}

impl From<[u8; 2]> for EtherType {
    fn from(value: [u8; 2]) -> Self {
        match value {
            [0x08, 0x00] => Self::IPv4,
            [0x08, 0x06] => Self::Arp,
            [0x86, 0xDD] => Self::IPv6,
            [0x88, 0xE1] => Self::HomePlugAV,
            _ => Self::Unknown(value),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Ethernet {
    dst_mac: MacAddress,
    src_mac: MacAddress,
    ether_type: EtherType,
}

impl Ethernet {
    const LEN: usize = 14;
}

impl Serialise for Ethernet {
    fn deserialise(buffer: &[u8]) -> Self {
        let mut ptr = 0;
        Self {
            dst_mac: consume!(ptr, buffer, [u8; 6]).into(),
            src_mac: consume!(ptr, buffer, [u8; 6]).into(),
            ether_type: consume!(ptr, buffer, [u8; 2]).into(),
        }
    }

    fn serialise(&self, buffer: &mut [u8]) {
        todo!()
    }
}

pub(super) struct Packet {
    ethernet: Ethernet,
}

impl Packet {}

impl Serialise for Packet {
    fn deserialise(buffer: &[u8]) -> Self {
        let ethernet = Ethernet::deserialise(&buffer[..Ethernet::LEN]);
        match &ethernet.ether_type {
            EtherType::Arp => {
                let arp = Arp::deserialise(
                    &buffer[Ethernet::LEN..Ethernet::LEN + Arp::LEN],
                );
                println!("{arp:X?}");
            }
            _ => {}
        }

        Self { ethernet }
    }

    fn serialise(&self, buffer: &mut [u8]) {
        todo!()
    }
}
