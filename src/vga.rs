use core::ptr::write_volatile;

// VGA Stuff
const BG_LIGHT_GREY: u8 = 0x07;
const BUFFER: *mut u8 = 0xB8000 as *mut u8;
const WIDTH: isize = 160;
static mut VGA_OFFSET: isize = 0;

pub(crate) struct Vga;

impl core::fmt::Write for Vga {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            for byte in s.bytes() {
                if byte == b'\n' {
                    VGA_OFFSET += WIDTH - (VGA_OFFSET % WIDTH);
                    continue;
                }
                write_volatile(BUFFER.offset(VGA_OFFSET), byte);
                write_volatile(BUFFER.offset(VGA_OFFSET + 1), BG_LIGHT_GREY);
                VGA_OFFSET = VGA_OFFSET + 2;
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
