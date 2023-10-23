use core::mem::size_of;

use alloc::vec::Vec;

use crate::cpu;

const BUSES: u8 = 255;
const SLOTS: u8 = 31;
const FUNCTIONS: u8 = 7;

#[repr(u16)]
pub enum Vendor {
    Intel = 0x8086,
}

#[derive(Debug, Clone, Copy)]
struct BaseAddress(u32);

impl BaseAddress {
    fn is_mmio(&self) -> bool {
        if self.0 & 1 == 1 {
            false
        } else {
            true
        }
    }

    fn ty(&self) -> u8 {
        (self.0 & 0b0110) as u8
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClassCode {
    Unclassified = 0x00,
    MassStorageController,
    NetworkController,
    DisplayController,
    MultumediaController,
    MemoryController,
    Bridge,
    SimpleCommunicationController,
    InputDeviceController,
    DockingStation,
    Processor,
    SerialBusController,
    WirelessController,
    IntelligentController,
    SatelliteCommunicationController,
    EncryptionController,
    SignalProcessingController,
    ProcessingAccelerator,
    NonEssentialInstrumentationator,
    Reserved0x3F,
    CoProcessor = 0x40,
    Reserved0xFE,
    UnassignedClass = 0xFF,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Header {
    pub vendor_id: u16,
    pub device_id: u16,
    command: u16,
    status: u16,
    revision_id: u8,
    prog_if: u8,
    subclass: u8,
    pub class_code: ClassCode,
    cache_line_size: u8,
    latency_timer: u8,
    header_type: u8,
    bist: u8,
    base_addrs: [BaseAddress; 5],
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
        let mut buffer = [0u32; size_of::<Self>() / size_of::<u32>()];

        for (i, bytes) in buffer.iter_mut().enumerate() {
            *bytes = Self::read32(bus, slot, function, i as u8 * 4);
        }

        let header = unsafe { *(buffer.as_ptr() as *const Header) };

        header
    }

    fn read32(bus: u8, slot: u8, function: u8, offset: u8) -> u32 {
        // Request info about PCI Device Header
        let request: u32 = Self::ENABLE_BIT
            | (bus as u32) << 16
            | (slot as u32) << 11
            | (function as u32) << 8
            | (offset as u32) & 0xFC;

        cpu::out32(Self::CONFIG_ADDR, request);

        cpu::in32(Self::CONFIG_DATA)
    }
}

fn get_devices() -> Vec<Header> {
    let mut devices: Vec<Header> = Vec::new();
    for bus in 0..=BUSES {
        for slot in 0..=SLOTS {
            for function in 0..=FUNCTIONS {
                let did_vid =
                    Header::read32(bus, slot, function, Header::DID_VID_OFFSET);
                let vendor = (did_vid >> 0) as u16;

                if vendor == !0 {
                    continue;
                }

                devices.push(Header::new(bus, slot, function));
            }
        }
    }
    devices
}

pub fn init() -> Vec<Header> {
    get_devices()
}
