use alloc::vec::Vec;

/// Consume a fixed slice from a buffer and increment the buffer
macro_rules! consume {
    // Parse slice into fixed length array
    ($ptr:expr, $buffer:expr, [$ty:ty; $len:expr]) => {{
        // Increment the pointer for size of type
        let mut tmp: [$ty; $len] = [0; $len];
        let start = $ptr;
        $ptr += core::mem::size_of::<[$ty; $len]>();

        // Create a buffer to receive the consumed bytes
        tmp.copy_from_slice(&$buffer[start..$ptr]);

        tmp
    }};

    // Parse u8
    ($ptr:expr, $buffer:expr, u8) => {{
        // Increment the pointer for size of type
        let start = $ptr;
        $ptr += core::mem::size_of::<u8>();

        $buffer[start]
    }};

    // Parse primatives
    ($ptr:expr, $buffer:expr, $endian:expr, $ty:ty) => {{
        // Increment the pointer for size of type
        let start = $ptr;
        $ptr += core::mem::size_of::<$ty>();

        let mut tmp = [0; core::mem::size_of::<$ty>()];
        tmp.copy_from_slice(&$buffer[start..$ptr]);

        match $endian {
            Endianness::Little => <$ty>::from_le_bytes(tmp),
            Endianness::Big => <$ty>::from_be_bytes(tmp),
        }
    }};
}

mod packet;

mod arp;
mod nic;
use crate::{net::nic::NetworkCard, pci};

enum Endianness {
    Big,
    Little,
}

trait Serialise {
    fn deserialise(buffer: &[u8]) -> Self;

    fn serialise(&self, buffer: &mut [u8]);
}

pub fn init(devices: &Vec<pci::Device>) {
    let nic = nic::find(devices).expect("No implented Network Cards found");
    nic.init();
}
