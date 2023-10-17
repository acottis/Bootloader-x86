use core::ptr::write_volatile;

const BG_LIGHT_GREY: u8 = 0x07;
const BUFFER: *mut u8 = 0xB8000 as *mut u8;
const WIDTH: isize = 160;
static mut OFFSET: isize = 0;

pub(crate) struct Vga;

impl core::fmt::Write for Vga {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            for byte in s.bytes() {
                if byte == b'\n' {
                    OFFSET += WIDTH - (OFFSET % WIDTH);
                    continue;
                }
                write_volatile(BUFFER.offset(OFFSET), byte);
                write_volatile(BUFFER.offset(OFFSET + 1), BG_LIGHT_GREY);
                OFFSET = OFFSET + 2;
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! write_vga {
    ($($arg:tt)*) => {
        _ = core::fmt::Write::write_fmt(&mut $crate::vga::Vga, format_args!($($arg)*));
    };
}
