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

    fn new() -> Self {
        let rsdp_ptr = RSDP::find().expect("No RSDP Found");
        let rsdp = unsafe { *(rsdp_ptr as *const RSDP) };
        rsdp.checksum(rsdp_ptr);

        rsdp
    }

    fn find() -> Option<usize> {
        for offset in (EBDA_START..EBDA_END).step_by(16) {
            let bytes = unsafe { *(offset as *const [u8; 8]) };
            if &bytes == RSDP::MAGIC.as_bytes() {
                return Some(offset);
            }
        }
        for offset in (REGION2_START..REGION2_END).step_by(16) {
            let bytes = unsafe { *(offset as *const [u8; 8]) };
            if &bytes == RSDP::MAGIC.as_bytes() {
                return Some(offset);
            }
        }
        None
    }

    fn checksum(&self, ptr: usize) {
        if self.revision != 0 {
            unimplemented!("Checksum for RSDP version {}", self.revision);
        }

        let raw_rsdp =
            unsafe { *(ptr as *const [u8; core::mem::size_of::<RSDP>()]) };

        let struct_bytes_sum = raw_rsdp
            .iter()
            .fold(0, |acc, byte| u8::wrapping_add(acc, *byte));
        if struct_bytes_sum != 0 {
            panic!("RDSP Checksum returned {}, should be 0", struct_bytes_sum);
        }
    }
}

pub fn init() {
    let rsdp = RSDP::new();

    println!("{rsdp:X?}");
}
