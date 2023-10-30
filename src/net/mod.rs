use alloc::vec::Vec;

mod packet;

mod nic;
use crate::{net::nic::NetworkCard, pci};

trait Serialise {
    fn deserialise(buffer: &[u8]) -> Self;

    fn serialise(&self, buffer: &mut [u8]);
}

pub fn init(devices: &Vec<pci::Device>) {
    let nic = nic::find(devices).expect("No implented Network Cards found");
    nic.init();
}
