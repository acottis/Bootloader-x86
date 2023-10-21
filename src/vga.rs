use core::{
    fmt::Write,
    ptr::write_volatile,
    sync::atomic::{AtomicIsize, Ordering},
};

const BUFFER: *mut u16 = 0xB8000 as *mut u16;
const WIDTH: isize = 80;
static OFFSET: AtomicIsize = AtomicIsize::new(0);

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

pub struct Vga;

impl Write for Vga {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            for byte in s.bytes() {
                let mut offset = OFFSET.load(Ordering::SeqCst);
                match byte {
                    b'\n' => offset += WIDTH - (offset % WIDTH),
                    BACKSPACE => {
                        offset -= 1;
                        write_volatile(
                            BUFFER.offset(offset),
                            (Colour::Black as u16) << 8 | byte as u16,
                        );
                    }
                    _ => {
                        write_volatile(
                            BUFFER.offset(offset),
                            (Colour::Green as u16) << 8 | byte as u16,
                        );
                        offset += 1;
                    }
                };
                OFFSET.store(offset, Ordering::SeqCst);
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
         _ = core::fmt::write(&mut $crate::vga::Vga, core::format_args!($($arg)*));
    };
}
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
         _ = core::fmt::write(&mut $crate::vga::Vga, core::format_args!($($arg)*));
         _ = core::fmt::write(&mut $crate::vga::Vga, core::format_args!("\n"));
    };
}
