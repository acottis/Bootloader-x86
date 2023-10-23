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

#[export_name = "memset"]
unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    s
}
