use alloc::vec::Vec;

mod nic;
use crate::{net::nic::NetworkCard, pci};

pub fn isr() {
    panic!("NIC");
}

pub fn init(devices: &Vec<pci::Device>) {
    let mut nic = nic::find(devices).expect("No implented Network Cards found");
    nic.init();
    loop {
        nic.receive();
    }
}
