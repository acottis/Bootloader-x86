pub const IDT_ENTRIES: u16 = 0xFF;
const IDT_SIZE: u16 = core::mem::size_of::<IdtEntry>() as u16;
const IDT_LENGTH: u16 = IDT_ENTRIES * IDT_SIZE - 1;
const CODE_SELECTOR_OFFSET: u16 = 8;

#[allow(dead_code)]
#[repr(C, packed)]
struct LidtDesc {
    limit: u16,
    base: u32,
}

#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Copy, Clone, Default)]
pub struct IdtEntry {
    isr_low: u16,
    kernel_cs: u16,
    reserved: u8,
    attributes: u8,
    isr_high: u16,
}

#[macro_export]
macro_rules! isr {
    ($irq:ident, $handler:ident) => {
        #[naked]
        unsafe extern "C" fn $irq() -> ! {
            core::arch::asm!(
                "pushad",
                "call {}",
                "popad",
                "iretd",
                sym $crate::$handler::isr,
                options(noreturn)
            );
        }
    };
}

#[macro_export]
macro_rules! trap_isr {
    ($irq:ident, $handler:ident) => {
        #[naked]
        unsafe extern "C" fn $irq() -> ! {
            core::arch::asm!(
                "pushad",
                "call {}",
                "popad",
                "cli",
                "hlt",
                sym $crate::$handler::trap,
                options(noreturn)
            );
        }
    };
}

trap_isr!(trap_default, interrupts);
isr!(isr_default, interrupts);
isr!(isr_0x21, keyboard);

pub unsafe fn isr() {
    crate::pic::end_of_interrupt();
}
pub unsafe fn trap() {
    crate::write_vga!("Error");
}

pub unsafe fn init_idt(idt: &mut [IdtEntry; IDT_ENTRIES as usize]) {
    let mut entry: usize = 0;
    while entry < IDT_ENTRIES as usize {
        if entry < 0x20 {
            // 0x0..0x1F to exception handlers
            idt[entry] = IdtEntry {
                isr_low: (trap_default as *const usize) as u16,
                // The entry of our CODE selector in GDT
                kernel_cs: CODE_SELECTOR_OFFSET,
                reserved: 0,
                attributes: 0x8F,
                isr_high: ((trap_default as *const usize as usize) >> 16) as u16,
            };
        } else if entry == 0x21 {
            // Keyboard
            idt[entry] = IdtEntry {
                isr_low: (isr_0x21 as *const usize) as u16,
                // The entry of our CODE selector in GDT
                kernel_cs: CODE_SELECTOR_OFFSET,
                reserved: 0,
                attributes: 0x8E,
                isr_high: ((isr_0x21 as *const usize as usize) >> 16) as u16,
            };
        } else {
            // Rest interupt handlers
            idt[entry] = IdtEntry {
                isr_low: (isr_default as *const usize) as u16,
                // The entry of our CODE selector in GDT
                kernel_cs: CODE_SELECTOR_OFFSET,
                reserved: 0,
                attributes: 0x8E,
                isr_high: ((isr_default as *const usize as usize) >> 16) as u16,
            };
        }
        entry += 1;
    }

    let lidt_desc = LidtDesc {
        limit: IDT_LENGTH,
        base: idt.as_ptr() as u32,
    };

    core::arch::asm!("lidt [{}]", in(reg) &lidt_desc);
}
