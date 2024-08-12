use crate::{
    cpu,
    pic::{end_of_interrupt, IRQ0_OFFSET},
};

const IDT_ENTRIES: u16 = 0xFF;
const IDT_SIZE: u16 = core::mem::size_of::<IdtEntry>() as u16;
const IDT_LENGTH: u16 = IDT_ENTRIES * IDT_SIZE - 1;
const CODE_SELECTOR_OFFSET: u16 = 8;
const IDT_BASE: usize = 0x1000;

const EXCEPTION_START: usize = 0x00;
const EXCEPTION_END: usize = 0x1F;

static mut IDT: *mut [IdtEntry; IDT_ENTRIES as usize] =
    IDT_BASE as *mut [IdtEntry; IDT_ENTRIES as usize];

/// ARG1: IRQ name
/// ARG2: module path
macro_rules! isr {
    ($irq:ident, $module:ident$(::$rest:ident)*) => {
        #[naked]
        unsafe extern "C" fn $irq() -> ! {
            core::arch::asm!(
                "call {}",
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
                "call {}",
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

fn isr() {
    end_of_interrupt();
}
fn trap(arg1: u32, arg2: u32, arg3: u32) {
    panic!("Exception - Stack: {arg1:x}, {arg2:x}, {arg3:x}");
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

pub struct Idt;

impl Idt {
    /// entry is the entry ignoring the exception offsets. So [IRQ0_OFFSET]
    /// is the first value
    pub fn insert(isr: unsafe extern "C" fn() -> !, entry: u8) {
        unsafe {
            (*IDT)[(entry + IRQ0_OFFSET) as usize] = IdtEntry {
                isr_low: isr as u16,
                // The entry of our CODE selector in GDT
                kernel_cs: CODE_SELECTOR_OFFSET,
                reserved: 0,
                attributes: 0x8F,
                isr_high: (isr as u32 >> 16) as u16,
            }
        }
    }

    pub fn init() {
        let mut entry: usize = 0;

        unsafe {
            while entry < IDT_ENTRIES as usize {
                match entry {
                    EXCEPTION_START..=EXCEPTION_END => {
                        (*IDT)[entry] = IdtEntry {
                            isr_low: trap_default as u16,
                            // The entry of our CODE selector in GDT
                            kernel_cs: CODE_SELECTOR_OFFSET,
                            reserved: 0,
                            attributes: 0x8F,
                            isr_high: (trap_default as u32 >> 16) as u16,
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
}
