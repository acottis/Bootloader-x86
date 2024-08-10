use core::{
    fmt::Write,
    ptr::write_volatile,
    sync::atomic::{AtomicIsize, Ordering},
};

const TEXT_BUF: *mut u16 = 0xB8000 as *mut u16;
const DRAW_BUF: *mut u8 = 0xA0000 as *mut u8;
const WIDTH: isize = 80;
static OFFSET: AtomicIsize = AtomicIsize::new(0);

const BACKSPACE: u8 = 0x08;

#[allow(dead_code)]
#[derive(Copy, Clone)]
#[repr(u8)]
enum Colour {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGrey,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightMagenta,
    Yellow,
    White,
}

pub struct Vga;

impl Write for Vga {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            let mut offset = OFFSET.load(Ordering::SeqCst);
            for byte in s.bytes() {
                match byte {
                    b'\n' => offset += WIDTH - (offset % WIDTH),
                    BACKSPACE => {
                        offset -= 1;
                        write_volatile(
                            TEXT_BUF.offset(offset),
                            (Colour::Black as u16) << 8 | byte as u16,
                        );
                    }
                    _ => {
                        write_volatile(
                            TEXT_BUF.offset(offset),
                            (Colour::Green as u16) << 8 | byte as u16,
                        );
                        offset += 1;
                    }
                };
            }
            OFFSET.store(offset, Ordering::SeqCst);
        }
        Ok(())
    }
}

pub fn draw() {
    draw_pixel(Coord::new(10, 20), Colour::Red);

    draw_line(Coord::new(30, 30), Coord::new(60, 30), Colour::Blue);
    draw_line(Coord::new(200, 130), Coord::new(250, 150), Colour::Blue);
}

struct Coord {
    x: u16,
    y: u16,
}

impl Coord {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

// Brensenham Algorithm
fn draw_line(start: Coord, end: Coord, colour: Colour) {
    let dx = end.x.abs_diff(start.x) as i32;
    let dy = end.y.abs_diff(start.y) as i32;

    let sx: i32 = if start.x < end.x { 1 } else { -1 };
    let sy: i32 = if start.y < end.y { 1 } else { -1 };

    let mut err = if dx > dy { dx } else { -dy } / 2;

    let mut x = start.x as i32;
    let mut y = start.y as i32;

    loop {
        draw_pixel(Coord::new(x as u16, y as u16), colour);
        if x as u16 == end.x && y as u16 == end.y {
            break;
        }
        let tmp_err = err;
        if tmp_err > -dx {
            err -= dy;
            x += sx;
        }
        if tmp_err < dy {
            err += dx;
            y += sy;
        }
    }
}

#[inline(always)]
fn draw_pixel(point: Coord, colour: Colour) {
    unsafe {
        write_volatile(
            DRAW_BUF.offset((point.x + 320 * point.y) as isize),
            colour as u8,
        )
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
