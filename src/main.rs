#![no_std]
#![no_main]
#![feature(naked_functions)]

extern crate alloc;
use alloc::{string::String, vec::Vec, *};

mod cpu;
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

    interrupts::init();
    pic::init();

    let mut foo: Vec<u32> = vec![1, 4, 59, 60];
    println!("{:X}", foo.as_mut_ptr() as u32);
    println!("{:X?}", foo);

    unsafe {
        let alloc_place = foo.as_ptr();
        println!("{:?}", *(alloc_place as *const [u8; 80]));
    }
    loop {
        cpu::halt();
    }
}
