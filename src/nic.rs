use alloc::vec::Vec;

use crate::pci::{self, Vendor};

struct NetworkCard {}

impl NetworkCard {
    const IMPLEMENTED: [(u16, u16); 1] = [(Vendor::Intel as u16, 0x100E)];

    fn new(device: &pci::Header) -> Self {
        Self {}
    }

    fn find(devices: &Vec<pci::Header>) -> Option<NetworkCard> {
        for device in devices {
            if device.class_code == pci::ClassCode::NetworkController {
                let implemented = Self::IMPLEMENTED
                    .contains(&(device.vendor_id, device.device_id));

                if implemented {
                    return Some(NetworkCard::new(device));
                }
            }
        }
        None
    }
}

pub fn isr() {
    panic!("NIC");
}

pub fn init(devices: &Vec<pci::Header>) {
    NetworkCard::find(devices);
}
