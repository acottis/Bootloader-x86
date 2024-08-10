use core::arch::asm;

#[inline(always)]
pub fn out8(port: u16, value: u8) {
    unsafe { asm!("out dx, al", in("al") value, in("dx") port) }
}

#[inline(always)]
pub fn out32(port: u16, value: u32) {
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

#[inline(always)]
pub fn cli() {
    unsafe { asm!("cli") }
}

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
