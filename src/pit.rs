//! Programmable Interval Timer
//! [https://wiki.osdev.org/Programmable_Interval_Timer]

use crate::cpu::{cli, halt, in8, out8, sti};
use core::sync::atomic::{AtomicU64, Ordering};

const CHANNEL_0: u16 = 0x40;
const COMMAND: u16 = 0x43;
const IRQ_PIN: u8 = 0;
const CLOCK_SPEED: u32 = 1193182;

#[allow(dead_code)]
#[repr(u8)]
enum Channel {
    Zero = 0,
    One,
    Two,
    ReadBack,
}

#[allow(dead_code)]
#[repr(u8)]
enum AccessMode {
    LowByte = 1,
    HighByte,
    BothBytes,
}

#[allow(dead_code)]
#[repr(u8)]
enum OperatingMode {
    Zero = 0,
    One,
    Two,
    Three,
    Four,
    Five,
}

static TICKS: AtomicU64 = AtomicU64::new(0);

isr!(irq, pit);
fn isr() {
    TICKS.fetch_add(1, Ordering::SeqCst);
    crate::pic::end_of_interrupt();
}

pub fn sleep_ms(ticks: u64) {
    let current_ticks = TICKS.load(Ordering::Relaxed);
    let target_ticks = current_ticks + ticks;
    while TICKS.load(Ordering::Relaxed) < target_ticks {
        halt();
    }
}

/// Hertz is the amount of times per second the interupt fires
pub fn init(hertz: u32) {
    cli();

    out8(
        COMMAND,
        (Channel::Zero as u8) << 6
            | (AccessMode::BothBytes as u8) << 4
            | (OperatingMode::Three as u8) << 1,
    );

    let divisor: u16 = (CLOCK_SPEED / hertz).try_into().unwrap_or(u16::MAX);

    out8(CHANNEL_0, divisor as u8);
    out8(CHANNEL_0, (divisor >> 8) as u8);

    crate::interrupts::Idt::insert(irq, IRQ_PIN);
    crate::pic::unmask(IRQ_PIN);
    sti();
}

fn read(channel: Channel) -> u16 {
    cli();

    out8(COMMAND, (channel as u8) << 6);
    let count = (in8(CHANNEL_0) as u16) | (in8(CHANNEL_0) as u16) << 8;

    sti();
    count
}
