#[derive(Debug)]
struct Entry {
    base_addr: u64,
    length: u64,
    r#type: u32,
    acpi_attributes: u32,
}

pub fn parse(memory_map: u32) {
    unsafe {
        let mm = &*(memory_map as *const [Entry; 20]);
        for entry in mm {
            if entry.r#type != 0 {
                crate::write_vga!("{:X?}\n", entry);
            }
        }
    }
}
