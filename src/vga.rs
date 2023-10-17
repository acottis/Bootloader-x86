use core::ptr::write_volatile;

const BUFFER: *mut u16 = 0xB8000 as *mut u16;
const WIDTH: isize = 80;
static mut OFFSET: isize = 0;

const BACKSPACE: u8 = 0x08;

#[repr(u8)]
#[allow(dead_code)]
enum Colour {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    White,
    Gray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    Yellow,
    BrightWhite,
}

pub(crate) struct Vga;

impl core::fmt::Write for Vga {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            for byte in s.bytes() {
                match byte {
                    b'\n' => {
                        OFFSET += WIDTH - (OFFSET % WIDTH);
                    }
                    BACKSPACE => {
                        OFFSET -= 1;
                        write_volatile(
                            BUFFER.offset(OFFSET),
                            (Colour::Black as u16) << 8 | byte as u16,
                        );
                    }
                    _ => {
                        write_volatile(
                            BUFFER.offset(OFFSET),
                            (Colour::Green as u16) << 8 | byte as u16,
                        );
                        OFFSET += 1;
                    }
                }
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
