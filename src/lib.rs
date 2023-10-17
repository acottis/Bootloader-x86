#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::asm;

mod cpu;
mod keyboard;
mod pic;
mod vga;

// IDT STUFF
const IDT_ENTRIES: u16 = 0xFF;
const IDT_SIZE: u16 = core::mem::size_of::<IdtEntry>() as u16;
const IDT_LENGTH: u16 = IDT_ENTRIES * IDT_SIZE - 1;
const CODE_SELECTOR_OFFSET: u16 = 8;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo<'_>) -> ! {
    write_vga!("Panic!");
    loop {}
}

#[allow(dead_code)]
#[repr(C, packed)]
struct LidtDesc {
    limit: u16,
    base: u32,
}

#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    isr_low: u16,
    kernel_cs: u16,
    reserved: u8,
    attributes: u8,
    isr_high: u16,
}

#[no_mangle]
unsafe extern "C" fn default_interrupt_handler() {
    pic::end_of_interrupt();
}

#[no_mangle]
unsafe extern "C" fn default_exception_handler() {
    write_vga!("Error");
}

#[naked]
unsafe extern "C" fn exception_handler() -> ! {
    asm!(
        "pushad",
        "call default_exception_handler",
        "popad",
        "cli",
        "hlt",
        options(noreturn)
    );
}

#[naked]
unsafe extern "C" fn interrupt_handler() -> ! {
    asm!(
        "pushad",
        "call default_interrupt_handler",
        "popad",
        "iretd",
        options(noreturn)
    );
}

#[naked]
unsafe extern "C" fn interrupt_handler0x21() -> ! {
    asm!(
        "pushad",
        "call irq0x21",
        "popad",
        "iretd",
        options(noreturn)
    );
}

fn print_stack(count: isize) {
    let stack_ptr: *const u32;
    unsafe { asm!("mov {}, esp", out(reg) stack_ptr) };
    for offset in 0..count {
        let value = unsafe { *stack_ptr.offset(offset) };
        write_vga!("ESP+{}: {:X}\n", offset * 4, value);
    }
}

unsafe fn setup_idt(idt: &mut [IdtEntry; IDT_ENTRIES as usize]) {
    let mut entry: usize = 0;
    while entry < IDT_ENTRIES as usize {
        if entry < 0x20 {
            // 0x0..0x1F to exception handlers
            idt[entry] = IdtEntry {
                isr_low: (exception_handler as *const usize) as u16,
                // The entry of our CODE selector in GDT
                kernel_cs: CODE_SELECTOR_OFFSET,
                reserved: 0,
                attributes: 0x8F,
                isr_high: ((exception_handler as *const usize as usize) >> 16) as u16,
            };
        } else if entry == 0x21 {
            // Keyboard
            idt[entry] = IdtEntry {
                isr_low: (interrupt_handler0x21 as *const usize) as u16,
                // The entry of our CODE selector in GDT
                kernel_cs: CODE_SELECTOR_OFFSET,
                reserved: 0,
                attributes: 0x8E,
                isr_high: ((interrupt_handler0x21 as *const usize as usize) >> 16) as u16,
            };
        } else {
            // Rest interupt handlers
            idt[entry] = IdtEntry {
                isr_low: (interrupt_handler as *const usize) as u16,
                // The entry of our CODE selector in GDT
                kernel_cs: CODE_SELECTOR_OFFSET,
                reserved: 0,
                attributes: 0x8E,
                isr_high: ((interrupt_handler as *const usize as usize) >> 16) as u16,
            };
        }
        entry += 1;
    }

    let lidt_desc = LidtDesc {
        limit: IDT_LENGTH,
        base: idt.as_ptr() as u32,
    };

    asm!("lidt [{}]", in(reg) &lidt_desc);
}

#[no_mangle]
unsafe fn entry() {
    let stack_ptr: u32;
    asm!("mov {:e}, esp", out(reg) stack_ptr);
    write_vga!("RustEntry ESP:{:X}\n", stack_ptr);

    let mut idt = [IdtEntry {
        isr_low: 0,
        kernel_cs: 0,
        reserved: 0,
        attributes: 0,
        isr_high: 0,
    }; IDT_ENTRIES as usize];
    setup_idt(&mut idt);
    pic::init();

    loop {
        asm!("hlt");
    }
}
