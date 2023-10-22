use core::mem::size_of;

use crate::cpu;

const BUSES: u8 = 255;
const SLOTS: u8 = 31;
const FUNCTIONS: u8 = 7;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Header {
    vendor_id: u16,
    device_id: u16,
    command: u16,
    status: u16,
    revision_id: u8,
    prog_if: u8,
    subclass: u8,
    class_code: u8,
    cache_line_size: u8,
    latency_timer: u8,
    header_type: u8,
    bist: u8,
    base_addr_0: u32,
    base_addr_1: u32,
    base_addr_2: u32,
    base_addr_3: u32,
    base_addr_4: u32,
    base_addr_5: u32,
    cardbus_cis_ptr: u32,
    subsystem_vendor_id: u16,
    subsystem_id: u16,
    expansion_rom_base_addr: u32,
    capabilities_ptr: u8,
    _reserved: [u8; 7],
    interrupt_line: u8,
    interrupt_pin: u8,
    min_grant: u8,
    max_latency: u8,
}

impl Header {
    const CONFIG_ADDR: u16 = 0xCF8;
    const CONFIG_DATA: u16 = 0xCFC;

    const ENABLE_BIT: u32 = 1 << 31;
    const DID_VID_OFFSET: u8 = 0;

    fn new(bus: u8, slot: u8, function: u8) -> Self {
        let mut buffer = [0u32; size_of::<Self>() / 4];

        for (i, bytes) in buffer.iter_mut().enumerate() {
            *bytes = Self::read32(bus, slot, function, i as u8 * 4);
        }

        let header = unsafe { *(buffer.as_ptr() as *const Header) };

        header
    }

    fn read32(bus: u8, slot: u8, function: u8, offset: u8) -> u32 {
        // Request info about PCI dev
        let request: u32 = Self::ENABLE_BIT
            | (bus as u32) << 16
            | (slot as u32) << 11
            | (function as u32) << 8
            | (offset as u32) & 0xFC;

        cpu::out32(Self::CONFIG_ADDR, request);

        cpu::in32(Self::CONFIG_DATA)
    }
}

pub fn init() {
    for bus in 0..=BUSES {
        for slot in 0..=SLOTS {
            for function in 0..=FUNCTIONS {
                let did_vid =
                    Header::read32(bus, slot, function, Header::DID_VID_OFFSET);
                let vendor = (did_vid >> 0) as u16;
                if vendor == !0 {
                    continue;
                }

                let device = Header::new(bus, slot, function);
                println!("{:X?}", device.class_code);
            }
        }
    }
}

//fn check_vendor(bus: u8, slot: u8) {
//    /* Try and read the first configuration register. Since there are no
//     * vendors that == 0xFFFF, it must be a non-existent device. */
//    if ((vendor = pciConfigReadWord(bus, slot, 0, 0)) != 0xFFFF) {
//       device = pciConfigReadWord(bus, slot, 0, 2);
//       . . .
//    } return (vendor);
//}
