use crate::{cpu::in8, pic};

const KEYBOARD_PORT: u16 = 0x60;

const BACKSPACE: u8 = 0x0E;
const LEFT_CTRL: u8 = 0x1D;
const LEFT_SHIFT: u8 = 0x2A;
const RIGHT_SHIFT: u8 = 0x36;
const LEFT_ALT: u8 = 0x38;
const CAPS_LOCK: u8 = 0x3A;

const SCAN_CODE: [char; 58] = [
    '\0', '!', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\u{08}', '\t', 'Q',
    'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '[', ']', '\n', '\0', 'A', 'S', 'D', 'F', 'G',
    'H', 'J', 'K', 'L', ';', '\'', '`', '\0', '\\', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '.', '/',
    '\0', '*', '\0', ' ', '\0',
];

pub fn isr() {
    let raw_key = in8(KEYBOARD_PORT);

    if raw_key < SCAN_CODE.len() as u8 {
        let key = SCAN_CODE.get(raw_key as usize).unwrap_or(&'!');

        crate::write_vga!("{}", key);
    }
    pic::end_of_interrupt();
}
