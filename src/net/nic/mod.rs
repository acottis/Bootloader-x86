//! Provides generic NetworkCard to OS for Network Card implementations

mod e1000;
use core::fmt::Debug;

use alloc::vec::Vec;

use crate::pci::{self, Id, Vendor};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct MacAddress([u8; 6]);

pub trait NetworkCard {
    fn new(device: &pci::Device) -> Self;
    fn init(&mut self);
    fn mac(&self) -> MacAddress;
    fn receive(&self);
}

pub fn find(
    devices: &Vec<pci::Device>,
) -> Option<&mut (impl NetworkCard + Debug)> {
    for device in devices {
        if !device.is_network_controller() {
            continue;
        }

        match (device.vendor(), device.id()) {
            (Vendor::Intel, Id::E1000) => unsafe {
                e1000::DRIVER.write(e1000::Driver::new(device));
                return Some(e1000::DRIVER.assume_init_mut());
            },
            _ => continue,
        }
    }
    None
}
