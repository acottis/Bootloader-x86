use alloc::vec::Vec;

mod nic;
use crate::{net::nic::NetworkCard, pci};

pub fn isr() {
    print!("P");
    crate::pic::end_of_interrupt();
}

pub fn init(devices: &Vec<pci::Device>) {
    let mut nic = nic::find(devices).expect("No implented Network Cards found");
    nic.init();
}
