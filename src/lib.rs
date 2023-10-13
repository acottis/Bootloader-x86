#![no_std]
#![no_main]

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

unsafe fn exception_handler() -> ! {
    write_vga(b'A');
    asm!("cli", "hlt");
    loop {}
}

unsafe fn write_vga(byte: u8) {
    core::ptr::write_volatile(VGA_BUFFER.offset(0), byte);
    core::ptr::write_volatile(VGA_BUFFER.offset(1), BG_LIGHT_GREY);
    VGA_BUFFER = VGA_BUFFER.add(2);
}

unsafe fn setup_idt() {
    const IDT_ENTRIES: u16 = 9;
    const IDT_SIZE: u16 = core::mem::size_of::<IdtEntry>() as u16;
    const IDT_LENGTH: u16 = IDT_ENTRIES * IDT_SIZE;
    const CODE_SELECTOR_OFFSET: u16 = 8;

    let idt = [IdtEntry {
        isr_low: (exception_handler as *const usize) as u16,
        // The entry of our CODE selector in GDT
        kernel_cs: CODE_SELECTOR_OFFSET,
        reserved: 0,
        attributes: 0x8E,
        isr_high: ((exception_handler as *const usize as usize) >> 16) as u16,
    }; IDT_ENTRIES as usize];

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
    }
    loop {}
}
