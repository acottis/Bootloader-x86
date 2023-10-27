use core::ptr::{read_volatile, write_volatile};

use self::reg::{rctl, RCTL};

use super::{MacAddress, NetworkCard};
use crate::pci::{self};
const PACKET_SIZE: u64 = 2048;

/// registers
#[allow(dead_code)]
mod reg {
    pub const CTRL: u32 = 0x0000;
    pub const STATUS: u32 = 0x0008;
    pub const IMS: u32 = 0x00D0;
    pub const RCTL: u32 = 0x0100;
    pub(super) mod rctl {
        pub const ENABLE: u32 = 1 << 1;
        pub const UNICAST_PROMISCUOUS: u32 = 1 << 3;
        pub const MULTICAST_PROMISCUOUS: u32 = 1 << 4;
        pub const ACCEPT_BROADCAST: u32 = 1 << 15;
        pub const STRIP_CRC: u32 = 1 << 26;
    }

    pub const RDBAL0: u32 = 0x2800;
    pub const RDBAH0: u32 = 0x2804;
    pub const RDLEN0: u32 = 0x2808;
    pub const RDH0: u32 = 0x2810;
    pub const RDT0: u32 = 0x2818;
    /// Mac Address Low
    pub const RAL: u32 = 0x5400;
    /// Mac Address High
    pub const RAH: u32 = 0x5404;
}

//// Register offsets of the E1000
//const REG_TDBAL: u32 = 0x3800;
//const REG_TDBAH: u32 = 0x3804;
//const REG_TDLEN: u32 = 0x3808;
//const REG_TDH: u32 = 0x3810;
//const REG_TDT: u32 = 0x3818;
//const REG_TCTL: u32 = 0x0400;

const RECEIVE_DESC_BASE_ADDRESS: u64 = 0x800000;
const RECEIVE_DESC_BUF_LENGTH: u32 = 8;
const RECEIVE_BASE_BUFFER_ADDRESS: u64 = 0x880000;
const RECEIVE_QUEUE_HEAD_START: u32 = 20;
const RECEIVE_QUEUE_TAIL_START: u32 = 4;

/// This struct is the receive descriptor format that stores the packet metadata
/// and the buffer points to the packet location in memory
#[derive(Debug, Default)]
#[repr(C)]
struct Rdesc {
    buffer: u64,
    len: u16,
    checksum: u16,
    status: u8,
    errors: u8,
    special: u16,
}

impl Driver {
    /// Read from a register offset in the MMIO buffer
    fn read(&self, register_offset: u32) -> u32 {
        unsafe {
            read_volatile((self.mmio_base + register_offset) as *const u32)
        }
    }
    /// Write to a register offset in the MMIO buffer
    fn write(&self, register_offset: u32, value: u32) {
        unsafe {
            write_volatile(
                (self.mmio_base + register_offset) as *mut u32,
                value,
            )
        };
    }

    pub fn init_recieve(&self) {
        // Set the Receive Descriptor Length
        self.write(reg::RDLEN0, RECEIVE_DESC_BUF_LENGTH << 8);

        // Set the Receive Descriptor Head/Tail
        self.write(reg::RDH0, RECEIVE_QUEUE_HEAD_START);
        self.write(reg::RDT0, RECEIVE_QUEUE_TAIL_START);

        // give them a size we want Set the Receive Descriptor Base Address
        self.write(reg::RDBAH0, (RECEIVE_DESC_BASE_ADDRESS >> 32) as u32);
        self.write(reg::RDBAL0, RECEIVE_DESC_BASE_ADDRESS as u32);

        self.write(
            RCTL,
            rctl::ENABLE
                | rctl::UNICAST_PROMISCUOUS
                | rctl::MULTICAST_PROMISCUOUS
                | rctl::ACCEPT_BROADCAST
                | rctl::STRIP_CRC,
        );

        // Zero out the chosen memory location and place the memory location for
        // the raw packets in the Recieve buffer field in the [`Rdesc`]
        // struct
        let rdesc_base_ptr = RECEIVE_DESC_BASE_ADDRESS as *mut Rdesc;
        for offset in 0..RECEIVE_DESC_BUF_LENGTH as isize {
            let rdesc = Rdesc {
                buffer: RECEIVE_BASE_BUFFER_ADDRESS
                    + (offset as u64 * PACKET_SIZE),
                ..Default::default()
            };
            unsafe {
                core::ptr::write(rdesc_base_ptr.offset(offset), rdesc);
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Driver {
    mmio_base: u32,
    io_base: usize,
    flash_base: usize,
    mac_addr: MacAddress,
}

impl NetworkCard for Driver {
    fn new(device: &pci::Device) -> Self {
        let mmio_base = ((device.base_addrs()[0]) & !0b1111) as u32;
        let io_base = (device.base_addrs()[1] & !0b0011) as usize;
        let flash_base = device.base_addrs()[2] as usize;

        device.enable();

        Self {
            mmio_base,
            io_base,
            flash_base,
            mac_addr: MacAddress([0u8; 6]),
        }
    }

    fn mac(&self) -> MacAddress {
        self.mac_addr
    }

    fn init(&mut self) {
        self.mac_addr = unsafe {
            read_volatile((self.mmio_base + reg::RAL) as *const MacAddress)
        };
        self.init_recieve();

        self.write(reg::IMS, 0x1F8DC);
        self.write(reg::IMS, 0xff & !4);
        println!("{:b}", self.read(reg::IMS));
        println!("{:b}", self.read(0xc0));
    }

    fn receive(&self) {
        let rdesc_base_ptr = RECEIVE_DESC_BASE_ADDRESS as *mut Rdesc;

        for offset in 0..RECEIVE_DESC_BUF_LENGTH as isize {
            unsafe {
                let mut rdesc: Rdesc =
                    core::ptr::read(rdesc_base_ptr.offset(offset));

                // A non zero status means a packet has arrived and is ready for
                // processing
                if rdesc.status != 0 {
                    print!("P");

                    rdesc.status = 0;
                    rdesc.len = 0;

                    core::ptr::write_volatile(
                        rdesc_base_ptr.offset(offset),
                        rdesc,
                    );

                    self.write(
                        reg::RDT0,
                        (self.read(reg::RDT0) + 1) % RECEIVE_DESC_BUF_LENGTH,
                    )
                }
            }
        }
    }
}
