use crate::{cpu::in8, pic};

const KEYBOARD_PORT: u16 = 0x60;
const KEY_RETURN: u8 = 0x1C;
const SCAN_CODE: [char; 5] = ['!', '!', '1', '2', '3'];

pub unsafe fn isr() {
    let raw_key = in8(KEYBOARD_PORT);
    let key = SCAN_CODE.get(raw_key as usize).unwrap_or(&'!');

    crate::write_vga!("{}", key);
    pic::end_of_interrupt();
}
