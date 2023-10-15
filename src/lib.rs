#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::arch::asm;

const BG_LIGHT_GREY: u8 = 0x07;
static mut VGA_BUFFER: *mut u8 = 0xB8000 as *mut u8;

// IDT STUFF
const IDT_ENTRIES: u16 = 256;
const IDT_SIZE: u16 = core::mem::size_of::<IdtEntry>() as u16;
const IDT_LENGTH: u16 = IDT_ENTRIES * IDT_SIZE - 1;
const CODE_SELECTOR_OFFSET: u16 = 8;

/// PIC Stuff
const PIC1: u16 = 0x20;
const PIC1_COMMAND: u16 = PIC1;
const PIC1_DATA: u16 = PIC1 + 1;

const PIC2: u16 = 0xA0;
const PIC2_COMMAND: u16 = PIC2;
const PIC2_DATA: u16 = PIC2 + 1;

const PIC_END_OF_INTERRUPT: u8 = 0x20;

const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;

const ICW4_8086: u8 = 0x01;

#[panic_handler]
fn panic_handler(_panic: &core::panic::PanicInfo<'_>) -> ! {
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
unsafe extern "C" fn i() {
    write_vga(b'I');
    //pic_end_of_interrupt();
}

#[no_mangle]
unsafe extern "C" fn e() {
    write_vga(b'E');
}

#[naked]
unsafe extern "C" fn exception_handler() -> ! {
    asm!("call e", "cli", "hlt", options(noreturn));
}

#[naked]
unsafe extern "C" fn interrupt_handler() -> ! {
    asm!("pushad", "call i", "popad", "iretd", options(noreturn));
}

#[inline(always)]
unsafe fn out8(port: u16, value: u8) {
    asm!("out dx, al", in("al") value, in("dx") port)
}

#[inline(always)]
unsafe fn in8(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", out("al") value, in("dx") port);
    value
}

unsafe fn write_vga(byte: u8) {
    core::ptr::write_volatile(VGA_BUFFER.offset(0), byte);
    core::ptr::write_volatile(VGA_BUFFER.offset(1), BG_LIGHT_GREY);
    VGA_BUFFER = VGA_BUFFER.add(2);
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

    asm!("lidt [{lidt_desc}]", lidt_desc = in(reg) &lidt_desc);
}

fn pic_end_of_interrupt() {
    unsafe {
        out8(PIC1_COMMAND, PIC_END_OF_INTERRUPT);
        out8(PIC2_COMMAND, PIC_END_OF_INTERRUPT);
    }
}

fn setup_pic() {
    unsafe {
        let pic1_mask = in8(PIC1_DATA);
        let pic2_mask = in8(PIC2_DATA);

        // Initialise the PIC
        out8(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
        out8(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);

        // Point the PIC to the IDT indices
        out8(PIC1_DATA, 0x20);
        out8(PIC2_DATA, 0x28);

        // Tell master that slave is at IRQ2
        out8(PIC1_DATA, 0b0000_0100);
        // Tell slave its cascae identity
        out8(PIC2_DATA, 0b0000_0010);

        out8(PIC1_DATA, ICW4_8086);
        out8(PIC2_DATA, ICW4_8086);

        out8(PIC1_DATA, pic1_mask);
        out8(PIC2_DATA, pic2_mask);
    }
}

#[no_mangle]
fn entry() {
    unsafe {
        write_vga(b'H');

        let mut idt = [IdtEntry {
            isr_low: 0,
            kernel_cs: 0,
            reserved: 0,
            attributes: 0,
            isr_high: 0,
        }; IDT_ENTRIES as usize];
        setup_idt(&mut idt);

        //setup_pic();
        //asm!("sti");
        asm!("int 0x20");
        asm!("int 0x20");
        //asm!("int 100");
        write_vga(b'Z');
        asm!("hlt");
    }
}
