#![no_std]
#![no_main]
#![feature(naked_functions)]

extern crate alloc;

#[macro_use]
mod vga;
#[macro_use]
mod interrupts;

mod acpi;
mod cpu;
mod error;
mod instrinsics;
mod keyboard;
mod mm;
mod net;
mod pci;
mod pic;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo<'_>) -> ! {
    print!("{}", info);
    loop {
        cpu::halt();
    }
}

#[export_name = "entry"]
fn entry(entry_addr: u32, memory_map_base_addr: u32) {
    println!("Rust Entry ESP:{:X}", entry_addr);

    // This sets the initial IDT, must happen first to avoid clobbering
    // other devices setting interrupts
    interrupts::init();

    mm::init(memory_map_base_addr)
        .expect("Failed to find suitable memory region for allocator");
    pic::init();

    let devices = pci::init();
    net::init(&devices);

    acpi::init();
    keyboard::init();

    loop {
        cpu::halt();
    }
}
