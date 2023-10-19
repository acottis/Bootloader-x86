#![no_std]
#![no_main]
#![feature(naked_functions)]

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
    let stack_ptr = cpu::esp() as *const u32;
    for offset in 0..count {
        let value = unsafe { *stack_ptr.offset(offset) };
        crate::write_vga!("ESP+{}: {:X}\n", offset * 4, value);
    }
}

#[no_mangle]
fn entry(memory_map: u32) {
    write_vga!("Rust Entry ESP:{:X}\n", cpu::esp());

    unsafe {
        let map = *(memory_map as *const [u32; 20]);
        write_vga!("{map:X?}");
    }

    let mut idt = [interrupts::IdtEntry::default(); interrupts::IDT_ENTRIES as usize];
    interrupts::init_idt(&mut idt);
    pic::init();

    loop {
        cpu::halt();
    }
}
