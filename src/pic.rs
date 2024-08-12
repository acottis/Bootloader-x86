use crate::cpu::{cli, in8, out8, sti};

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;

const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const END_OF_INTERRUPT: u8 = 0x20;

const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;

const ICW4_8086: u8 = 0x01;

pub const IRQ0_OFFSET: u8 = 0x20;
const IRQ8_OFFSET: u8 = 0x28;

#[allow(dead_code)]
const READ_IRR: u8 = 0x0A;
#[allow(dead_code)]
const READ_ISR: u8 = 0x0B;

pub fn end_of_interrupt() {
    out8(PIC1_COMMAND, END_OF_INTERRUPT);
    out8(PIC2_COMMAND, END_OF_INTERRUPT);
}

/// Unmask a specific IRQ on the PIC
pub fn unmask(irq_pin: u8) {
    cli();
    if irq_pin < 8 {
        let mut mask = in8(PIC1_DATA);
        mask &= !(1 << irq_pin);
        out8(PIC1_DATA, mask);
    } else if irq_pin < 16 {
        let mut mask = in8(PIC2_DATA);
        mask &= !(1 << irq_pin);
        out8(PIC2_DATA, mask);
    } else {
        panic!("[ERROR] Invalid IRQ pin {irq_pin}, must be less than 16");
    }
    sti();
}

pub fn init() {
    cli();

    // Initialise the PIC
    out8(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
    out8(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);

    // Point the PIC to the IDT indices
    out8(PIC1_DATA, IRQ0_OFFSET);
    out8(PIC2_DATA, IRQ8_OFFSET);

    // Tell master that slave is at IRQ2
    out8(PIC1_DATA, 0b0000_0100);
    // Tell slave its cascade identity
    out8(PIC2_DATA, 0b0000_0010);

    // Set 8086 mode
    out8(PIC1_DATA, ICW4_8086);
    out8(PIC2_DATA, ICW4_8086);

    // Initialise all interupt pins as disabled, except from cascade
    // pin that essentially enables PIC2 PINS.
    // To register call the [unmask_irq()] function
    out8(PIC1_DATA, 0b1111_1011);
    out8(PIC2_DATA, 0b1111_1111);

    sti();
}
