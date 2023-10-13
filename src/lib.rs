#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::asm;

const BG_LIGHT_GREY: u8 = 0x07;
static mut VGA_BUFFER: *mut u8 = 0xB8000 as *mut u8;

#[panic_handler]
fn panic_handler(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[repr(packed)]
struct LidtDesc {
    limit: u16,
    base: u32,
}

#[repr(packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    isr_low: u16,
    kernel_cs: u16,
    reserved: u8,
    attributes: u8,
    isr_high: u16,
}

#[no_mangle]
unsafe fn i() {
    write_vga(b'I');
}

#[no_mangle]
unsafe fn e() {
    write_vga(b'E');
}

#[naked]
unsafe extern "C" fn exception_handler() -> ! {
    asm!("call e", "cli", "hlt", options(noreturn));
}

#[naked]
unsafe extern "C" fn interrupt_handler() -> ! {
    asm!("call i", "ret", options(noreturn));
}

unsafe fn write_vga(byte: u8) {
    core::ptr::write_volatile(VGA_BUFFER.offset(0), byte);
    core::ptr::write_volatile(VGA_BUFFER.offset(1), BG_LIGHT_GREY);
    VGA_BUFFER = VGA_BUFFER.add(2);
}

unsafe fn setup_idt() {
    const IDT_ENTRIES: u16 = 256;
    const IDT_SIZE: u16 = core::mem::size_of::<IdtEntry>() as u16;
    const IDT_LENGTH: u16 = IDT_ENTRIES * IDT_SIZE;
    const CODE_SELECTOR_OFFSET: u16 = 8;

    // First 32 set to exception handlers
    let mut idt = [IdtEntry {
        isr_low: (exception_handler as *const usize) as u16,
        // The entry of our CODE selector in GDT
        kernel_cs: CODE_SELECTOR_OFFSET,
        reserved: 0,
        attributes: 0x8E,
        isr_high: ((exception_handler as *const usize as usize) >> 16) as u16,
    }; IDT_ENTRIES as usize];

    // Set the remaining to interrupt handlers
    let mut counter = IDT_ENTRIES as usize - 1;
    while counter > 32 {
        idt[counter] = IdtEntry {
            isr_low: (interrupt_handler as *const usize) as u16,
            // The entry of our CODE selector in GDT
            kernel_cs: CODE_SELECTOR_OFFSET,
            reserved: 0,
            attributes: 0x8E,
            isr_high: ((interrupt_handler as *const usize as usize) >> 16) as u16,
        };
        counter -= 1;
    }

    let lidt_desc = LidtDesc {
        limit: IDT_LENGTH,
        base: idt.as_ptr() as u32,
    };

    asm!("lidt [{lidt_desc}]", lidt_desc = in(reg) &lidt_desc);
    asm!("sti");
}

#[no_mangle]
fn entry() {
    unsafe {
        write_vga(b'H');
        setup_idt();
        asm!("int 0x30");
    }
    loop {}
}
