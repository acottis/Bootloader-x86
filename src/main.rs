#![no_std]
#![no_main]
#![feature(naked_functions)]

extern crate alloc;
use alloc::{string::String, vec::Vec, *};

mod cpu;
mod instrinsics;
mod interrupts;
mod keyboard;
mod mm;
mod pic;
mod vga;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("{:?}", info);
    loop {
        cpu::halt();
    }
}

#[no_mangle]
pub fn entry(memory_map_base_addr: u32) {
    println!("Rust Entry ESP:{:X}", cpu::esp());

    mm::init(memory_map_base_addr)
        .expect("Failed to find suitable memory region for allocator");

    let t = Vec::from([b'A' as u64; 500]);
    crate::println!("{}", t[40]);

    interrupts::init();
    pic::init();

    loop {
        cpu::halt();
    }
}
