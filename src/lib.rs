#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::{arch::asm, ptr::write_volatile};

// VGA Stuff
const BG_LIGHT_GREY: u8 = 0x07;
const VGA_BUFFER: *mut u8 = 0xB8000 as *mut u8;
const VGA_WIDTH: isize = 160;
static mut VGA_OFFSET: isize = 0;

// IDT STUFF
const IDT_ENTRIES: u16 = 0xFF;
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
unsafe extern "C" fn i() {
    print_stack(20);
    asm!("cli", "hlt");
    pic_eoi();
}

#[no_mangle]
unsafe extern "C" fn e() {
    write_vga!("Error");
}

#[naked]
unsafe extern "C" fn exception_handler() -> ! {
    asm!("pushad", "call e", "popad", "cli", "hlt", options(noreturn));
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

struct Vga;

impl core::fmt::Write for Vga {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            for byte in s.bytes() {
                if byte == b'\n' {
                    VGA_OFFSET += VGA_WIDTH - (VGA_OFFSET % VGA_WIDTH);
                    continue;
                }
                write_volatile(VGA_BUFFER.offset(VGA_OFFSET), byte);
                write_volatile(VGA_BUFFER.offset(VGA_OFFSET), byte);
                write_volatile(VGA_BUFFER.offset(VGA_OFFSET + 1), BG_LIGHT_GREY);
                VGA_OFFSET = VGA_OFFSET + 2;
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! write_vga {
    ($($arg:tt)*) => {
        _ = core::fmt::Write::write_fmt(&mut Vga, format_args!($($arg)*));
    };
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

unsafe fn pic_eoi() {
    out8(PIC1_COMMAND, PIC_END_OF_INTERRUPT);
    out8(PIC2_COMMAND, PIC_END_OF_INTERRUPT);
}

unsafe fn setup_pic() {
    // Save default mask
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

    // Set 8086 mode
    out8(PIC1_DATA, ICW4_8086);
    out8(PIC2_DATA, ICW4_8086);

    // Use default masks
    out8(PIC1_DATA, pic1_mask);
    out8(PIC2_DATA, pic2_mask);
    asm!("sti");
}

#[no_mangle]
unsafe fn entry() {
    let stack_ptr: u32;
    asm!("mov {}, esp", out(reg) stack_ptr);
    write_vga!("RustEntry ESP:{:X}\n", stack_ptr);

    let mut idt = [IdtEntry {
        isr_low: 0,
        kernel_cs: 0,
        reserved: 0,
        attributes: 0,
        isr_high: 0,
    }; IDT_ENTRIES as usize];
    setup_idt(&mut idt);
    setup_pic();

    loop {
        asm!("hlt");
    }
}
