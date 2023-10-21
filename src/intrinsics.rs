#[export_name = "__udivdi3"]
pub extern "C" fn __udivdi3(n: u64, d: u64) -> u64 {
    panic!("udivdi3");
}

#[export_name = "memcpy"]
unsafe fn memcpy(dst: *mut u8, src: *mut u8, n: usize) -> *mut u8 {
    core::ptr::copy_nonoverlapping(src, dst, n);
    dst
}
