use core::sync::atomic::{AtomicBool, Ordering};

use crate::{cpu::in8, pic};

const KEYBOARD_PORT: u16 = 0x60;

const BACKSPACE: u8 = 0x0E;

const LEFT_SHIFT_PRESSED: u8 = 0x2A;
const LEFT_SHIFT_RELEASED: u8 = LEFT_SHIFT_PRESSED + RELEASE_OFFSET;
const RIGHT_SHIFT_PRESSED: u8 = 0x36;
const RIGHT_SHIFT_RELEASED: u8 = RIGHT_SHIFT_PRESSED + RELEASE_OFFSET;
const LEFT_ALT: u8 = 0x38;
const CAPS_LOCK: u8 = 0x3A;
const LEFT_CTRL: u8 = 0x1D;

const RELEASE_OFFSET: u8 = 0x80;

static LEFT_SHIFT_DOWN: AtomicBool = AtomicBool::new(false);

const KEY_MAP: [char; 59] = [
    '\0', '\0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', '\u{08}', '\t', 'q',
    'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n', '\0', 'a', 's', 'd', 'f', 'g',
    'h', 'j', 'k', 'l', ';', '\'', '`', '\0', '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.',
    '/', '\0', '\0', '\0', ' ', '\0',
];

const SHIFT_KEY_MAP: [char; 59] = [
    '\0', '\0', '!', '"', '£', '$', '%', '^', '&', '*', '(', ')', '_', '+', '\u{08}', '\t', 'Q',
    'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', '\n', '\0', 'A', 'S', 'D', 'F', 'G',
    'H', 'J', 'K', 'L', ':', '|', '¬', '\0', '\\', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>',
    '?', '\0', '\0', '\0', ' ', '\0',
];

pub fn isr() {
    let raw_key = in8(KEYBOARD_PORT);

    match raw_key {
        LEFT_SHIFT_PRESSED => LEFT_SHIFT_DOWN.store(true, Ordering::Relaxed),
        LEFT_SHIFT_RELEASED => LEFT_SHIFT_DOWN.store(false, Ordering::Relaxed),
        raw_key => {
            let key = match LEFT_SHIFT_DOWN.load(Ordering::Relaxed) {
                true => SHIFT_KEY_MAP.get(raw_key as usize).unwrap_or(&'\0'),
                false => KEY_MAP.get(raw_key as usize).unwrap_or(&'\0'),
            };

            if key != &'\0' {
                crate::write_vga!("{}", key);
            }
        }
    };

    pic::end_of_interrupt();
}
