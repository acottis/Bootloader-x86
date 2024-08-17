use core::arch::asm;

struct Ax {}

#[derive(Default)]
#[repr(C)]
pub struct Registers {
    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,
    pub di: u16,
    pub si: u16,
    pub sp: u16,
    pub bp: u16,
}

extern "C" {
    /// Invokes an interupt in real mode before comming back to protected
    pub fn invoke_realmode(int: u16, registers: *mut Registers);
}

/// If we send IO port instructions too quickly we have timing issues
/// https://wiki.osdev.org/Inline_Assembly/Examples#I/O_access
#[inline(always)]
pub fn iowait() {
    out8_fast(0x80, 0);
}

#[inline(always)]
pub fn out8(port: u16, value: u8) {
    out8_fast(port, value);
    iowait();
}

#[inline(always)]
fn out8_fast(port: u16, value: u8) {
    unsafe { asm!("out dx, al", in("al") value, in("dx") port) }
}

#[inline(always)]
pub fn out32(port: u16, value: u32) {
    out32_fast(port, value);
    iowait();
}

#[inline(always)]
fn out32_fast(port: u16, value: u32) {
    unsafe { asm!("out dx, eax", in("eax") value, in("dx") port) }
}

#[inline(always)]
pub fn in8(port: u16) -> u8 {
    unsafe {
        let value: u8;
        asm!("in al, dx", out("al") value, in("dx") port);
        value
    }
}

#[inline(always)]
pub fn in32(port: u16) -> u32 {
    unsafe {
        let value: u32;
        asm!("in eax, dx", out("eax") value, in("dx") port);
        value
    }
}

/// Clear the interrupt flag [https://www.felixcloutier.com/x86/cli]
#[inline(always)]
pub fn cli() {
    unsafe { asm!("cli") }
}

/// Set the interrupt flag [https://www.felixcloutier.com/x86/sti]
#[inline(always)]
pub fn sti() {
    unsafe { asm!("sti") }
}

#[allow(dead_code)]
#[inline(always)]
pub fn esp() -> u32 {
    let esp: u32;
    unsafe { asm!("mov {:e}, esp", out(reg) esp) }
    esp
}

#[inline(always)]
pub fn lidt(descriptor: &crate::interrupts::LidtDesc) {
    unsafe {
        core::arch::asm!("lidt [{}]", in(reg) descriptor);
    }
}

#[inline(always)]
pub fn halt() {
    unsafe { core::arch::asm!("hlt") }
}
