use core::arch::asm;

#[inline(always)]
pub unsafe fn out8(port: u16, value: u8) {
    asm!("out dx, al", in("al") value, in("dx") port)
}

#[inline(always)]
pub unsafe fn in8(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", out("al") value, in("dx") port);
    value
}
