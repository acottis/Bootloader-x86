#![no_std]
#![no_main]
#![feature(naked_functions)]

extern crate alloc;
use alloc::{string::String, *};

mod cpu;
mod interrupts;
mod intrinsics;
mod keyboard;
mod mm;
mod pic;
mod vga;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo<'_>) -> ! {
    println!("{:?}", info);
    loop {}
}

#[export_name = "entry"]
fn entry(memory_map_base_addr: u32) {
    println!("Rust Entry ESP:{:X}", cpu::esp());

    mm::init(memory_map_base_addr)
        .expect("Failed to find suitable memory region for allocator");

    interrupts::init();
    pic::init();

    let mut foo = vec![1u8, 2, 3, 4, 5];
    let mut bar = vec![1u32, 2, 3, 4, 5];
    let mut baz = String::from("haha");

    println!("{:X}", baz.as_mut_ptr() as u32);
    println!("{:X}", foo.as_mut_ptr() as u32);
    println!("{:X}", bar.as_mut_ptr() as u32);
    println!("{}", baz);

    unsafe {
        println!("{:?}", *(0x100000 as *const [u8; 80]));
    }
    loop {
        cpu::halt();
    }
}
