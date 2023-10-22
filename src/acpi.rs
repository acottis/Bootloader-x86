use crate::error::{Error, Result};
use core::mem::size_of;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(packed)]
struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_addr: u32,
}

impl Rsdp {
    const MAGIC: &'static str = "RSD PTR ";

    const REGION1_START: usize = 0x40E;
    const REGION1_END: usize = Self::REGION1_START + 1024;

    const REGION2_START: usize = 0xE0000;
    const REGION2_END: usize = 0xFFFFF;

    fn new() -> Result<Self> {
        let rsdp_ptr = Rsdp::find()?;

        let rsdp = unsafe { *(rsdp_ptr as *const Rsdp) };

        rsdp.checksum(rsdp_ptr)?;

        Ok(rsdp)
    }

    fn find() -> Result<usize> {
        for offset in (Self::REGION1_START..Self::REGION1_END).step_by(16) {
            let bytes = unsafe { *(offset as *const [u8; 8]) };
            if &bytes == Rsdp::MAGIC.as_bytes() {
                return Ok(offset);
            }
        }
        for offset in (Self::REGION2_START..Self::REGION2_END).step_by(16) {
            let bytes = unsafe { *(offset as *const [u8; 8]) };
            if &bytes == Rsdp::MAGIC.as_bytes() {
                return Ok(offset);
            }
        }
        Err(Error::CannotFindRsdp)
    }

    fn checksum(&self, ptr: usize) -> Result<()> {
        if self.revision != 0 {
            unimplemented!("Checksum for RSDP version {}", self.revision);
        }

        let raw_rsdp = unsafe { *(ptr as *const [u8; size_of::<Self>()]) };

        let struct_bytes_sum = raw_rsdp
            .iter()
            .fold(0, |acc, byte| u8::wrapping_add(acc, *byte));

        if struct_bytes_sum != 0 {
            return Err(Error::RsdpCheckSumNotZero(struct_bytes_sum));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct Rsdt {
    acpi_sdt_header: AcpiSdtHeader,
    std_ptrs: [u32; 20],
}
impl Rsdt {
    fn new(rsdp: &Rsdp) -> Result<Self> {
        let rsdt = unsafe { *(rsdp.rsdt_addr as *const Rsdt) };
        rsdt.checksum(rsdp.rsdt_addr)?;

        Ok(rsdt)
    }

    fn checksum(&self, ptr: u32) -> Result<()> {
        // Sum all bytes with wrapping_add (lowest byte should be 0)
        let sum = (0..self.acpi_sdt_header.length).fold(0, |acc, offset| {
            let byte = unsafe { *(ptr as *const u8).offset(offset as isize) };
            u8::wrapping_add(byte, acc)
        });

        if sum != 0 {
            return Err(Error::RsdtCheckSumNotZero(sum));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct AcpiSdtHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 6],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

pub fn init() {
    let rsdp = Rsdp::new().unwrap();

    let rsdt = Rsdt::new(&rsdp).unwrap();

    let entries =
        (rsdt.acpi_sdt_header.length as usize - size_of::<AcpiSdtHeader>()) / 4;

    for entry in 0..entries {
        let h = unsafe { *(rsdt.std_ptrs[entry] as *const AcpiSdtHeader) };
        println!("{h:X?}");
        println!("{:?}", core::str::from_utf8(&h.signature));
    }
}
