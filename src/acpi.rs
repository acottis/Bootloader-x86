const EBDA_START: usize = 0x40E;
const EBDA_END: usize = EBDA_START + 1024;

const REGION2_START: usize = 0xE0000;
const REGION2_END: usize = 0xFFFFF;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(packed)]
struct RSDP {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_addr: u32,
}

impl RSDP {
    const MAGIC: &'static str = "RSD PTR ";

    fn handle(ptr: usize) {
        let rdsp = unsafe { *(ptr as *const RSDP) };
        rdsp.checksum();
    }

    fn checksum(&self) {
        println!("{self:#X?}");
    }
}

pub fn init() {
    println!("Looking for {:X?}", b"RSD PTR ");
    for offset in (EBDA_START..EBDA_END).step_by(16) {
        let bytes = unsafe { *(offset as *const [u8; 8]) };
        if &bytes == RSDP::MAGIC.as_bytes() {
            RSDP::handle(offset);
        }
    }
    for offset in (REGION2_START..REGION2_END).step_by(16) {
        let bytes = unsafe { *(offset as *const [u8; 8]) };
        if &bytes == RSDP::MAGIC.as_bytes() {
            RSDP::handle(offset);
        }
    }
}
