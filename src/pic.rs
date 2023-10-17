use crate::cpu::{cli, in8, out8, sti};

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

pub fn end_of_interrupt() {
    out8(PIC1_COMMAND, PIC_END_OF_INTERRUPT);
    out8(PIC2_COMMAND, PIC_END_OF_INTERRUPT);
}

pub fn init() {
    cli();
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
    sti();
}
