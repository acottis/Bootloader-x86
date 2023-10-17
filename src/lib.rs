#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::asm;

mod cpu;
mod interrupts;
mod keyboard;
mod pic;
mod vga;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo<'_>) -> ! {
    write_vga!("Panic!");
    loop {}
}

fn print_stack(count: isize) {
    let stack_ptr: *const u32;
    unsafe { asm!("mov {}, esp", out(reg) stack_ptr) };
    for offset in 0..count {
        let value = unsafe { *stack_ptr.offset(offset) };
        crate::write_vga!("ESP+{}: {:X}\n", offset * 4, value);
    }
}

#[no_mangle]
unsafe fn entry() {
    let stack_ptr: u32;
    asm!("mov {:e}, esp", out(reg) stack_ptr);
    write_vga!("RustEntry ESP:{:X}\n", stack_ptr);

    let mut idt = [interrupts::IdtEntry::default(); interrupts::IDT_ENTRIES as usize];
    interrupts::init_idt(&mut idt);
    pic::init();

    loop {
        asm!("hlt");
    }
}
