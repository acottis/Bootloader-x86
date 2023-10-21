#[export_name = "memcpy"]
unsafe fn memcpy(dst: *mut u8, src: *mut u8, n: usize) -> *mut u8 {
    core::ptr::copy_nonoverlapping(src, dst, n);
    dst
}
