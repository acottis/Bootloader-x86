#![no_std]
#![no_main]
#![feature(naked_functions)]

extern crate alloc;

#[macro_use]
mod vga;

mod acpi;
mod cpu;
mod error;
mod instrinsics;
mod interrupts;
mod keyboard;
mod mm;
mod pic;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo<'_>) -> ! {
    print!("{}", info);
    loop {
        cpu::halt();
    }
}

#[export_name = "entry"]
fn entry(memory_map_base_addr: u32) {
    println!("Rust Entry ESP:{:X}", cpu::esp());

    mm::init(memory_map_base_addr)
        .expect("Failed to find suitable memory region for allocator");
    interrupts::init();
    pic::init();
    acpi::init();

    loop {
        cpu::halt();
    }
}
