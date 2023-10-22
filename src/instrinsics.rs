#[export_name = "memcpy"]
unsafe extern "C" fn memcpy(
    dest: *mut u8,
    src: *const u8,
    n: usize,
) -> *mut u8 {
    for b in 0..n {
        *dest.offset(b as isize) = *src.offset(b as isize)
    }
    dest
}
