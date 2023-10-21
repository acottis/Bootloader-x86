use crate::{cpu, pic::end_of_interrupt};

const IDT_ENTRIES: u16 = 0xFF;
const IDT_SIZE: u16 = core::mem::size_of::<IdtEntry>() as u16;
const IDT_LENGTH: u16 = IDT_ENTRIES * IDT_SIZE - 1;
const CODE_SELECTOR_OFFSET: u16 = 8;

static mut IDT: *mut [IdtEntry; IDT_ENTRIES as usize] =
    0x1000 as *mut [IdtEntry; IDT_ENTRIES as usize];

macro_rules! isr {
    ($irq:ident, $module:ident$(::$rest:ident)*) => {
        #[naked]
        unsafe extern "C" fn $irq() -> ! {
            core::arch::asm!(
                "pushad",
                "call {}",
                "popad",
                "iretd",
                sym $crate::$module$(::$rest)*::isr,
                options(noreturn)
            );
        }
    };
}

macro_rules! trap_isr {
    ($irq: ident, $module: ident$(::$rest: ident)*) => {
        #[naked]
        unsafe extern "C" fn $irq() -> ! {
            core::arch::asm!(
                "pushad",
                "call {}",
                "popad",
                "cli",
                "hlt",
                sym $crate::$module$(::$rest)*::trap,
                options(noreturn)
            );
        }
    };
}

trap_isr!(trap_default, interrupts);
isr!(isr_default, interrupts);
isr!(isr_0x21, keyboard);

pub fn isr() {
    end_of_interrupt();
}
pub fn trap() {
    crate::println!("Error");
}

#[allow(dead_code)]
#[repr(packed)]
pub struct LidtDesc {
    limit: u16,
    base: u32,
}

#[allow(dead_code)]
#[repr(packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    isr_low: u16,
    kernel_cs: u16,
    reserved: u8,
    attributes: u8,
    isr_high: u16,
}

impl Default for IdtEntry {
    fn default() -> Self {
        Self {
            isr_low: (isr_default as *const usize) as u16,
            // The entry of our CODE selector in GDT
            kernel_cs: CODE_SELECTOR_OFFSET,
            reserved: 0,
            attributes: 0x8E,
            isr_high: ((isr_default as *const usize as usize) >> 16) as u16,
        }
    }
}

pub fn init() {
    const EXCEPTION_START: usize = 0x00;
    const EXCEPTION_END: usize = 0x1F;
    const KEYBOARD_IRQ: usize = 0x21;
    let mut entry: usize = 0;

    unsafe {
        while entry < IDT_ENTRIES as usize {
            match entry {
                EXCEPTION_START..=EXCEPTION_END => {
                    (*IDT)[entry] = IdtEntry {
                        isr_low: (trap_default as *const usize) as u16,
                        // The entry of our CODE selector in GDT
                        kernel_cs: CODE_SELECTOR_OFFSET,
                        reserved: 0,
                        attributes: 0x8F,
                        isr_high: ((trap_default as *const usize as usize)
                            >> 16) as u16,
                    };
                }
                KEYBOARD_IRQ => {
                    (*IDT)[entry] = IdtEntry {
                        isr_low: (isr_0x21 as *const usize) as u16,
                        // The entry of our CODE selector in GDT
                        kernel_cs: CODE_SELECTOR_OFFSET,
                        reserved: 0,
                        attributes: 0x8E,
                        isr_high: ((isr_0x21 as *const usize as usize) >> 16)
                            as u16,
                    };
                }
                _ => (*IDT)[entry] = IdtEntry::default(),
            };
            entry += 1;
        }
        let lidt_desc = LidtDesc {
            limit: IDT_LENGTH,
            base: IDT as u32,
        };

        cpu::lidt(&lidt_desc);
    }
}
