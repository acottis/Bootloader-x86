//! Programmable Interval Timer
//! [https://wiki.osdev.org/Programmable_Interval_Timer]

use crate::cpu::{cli, in8, out8, sti};

const CHANNEL_0: u16 = 0x40;
const COMMAND: u16 = 0x43;
const IRQ_PIN: u8 = 0;

isr!(irq, pit);
fn isr() {
    crate::pic::end_of_interrupt();
}

pub fn init() {
    cli();
    out8(COMMAND, 0b00110100);

    let frequency: u32 = 1000;
    let divisor: u32 = 1193182 / frequency;

    out8(CHANNEL_0, divisor as u8);
    out8(CHANNEL_0, (divisor >> 8) as u8);

    crate::interrupts::Idt::insert(irq, IRQ_PIN);
    crate::pic::unmask(IRQ_PIN);
    sti();
}

pub fn read() -> u16 {
    cli();

    out8(COMMAND, 0);
    let count = (in8(CHANNEL_0) as u16) | (in8(CHANNEL_0) as u16) << 8;

    sti();
    count
}
