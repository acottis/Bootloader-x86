use core::{
    alloc::GlobalAlloc,
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
};

#[derive(Debug)]
struct GlobalAllocator {
    arena: UnsafeCell<*mut u8>,
    remaining: AtomicUsize,
}

#[global_allocator]
static mut GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator {
    arena: UnsafeCell::new(core::ptr::null_mut()),
    remaining: AtomicUsize::new(0),
};

unsafe impl Sync for GlobalAllocator {}

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if layout.align() == 0 {
            return core::ptr::null_mut();
        }

        let mut alloc_base = 0;
        if self
            .remaining
            .fetch_update(SeqCst, SeqCst, |mut remaining| {
                remaining -= layout.size();
                alloc_base = remaining & (!layout.align() - 1);
                Some(alloc_base)
            })
            .is_err()
        {
            return core::ptr::null_mut();
        }

        (*self.arena.get()).add(alloc_base)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {}
}

#[allow(dead_code)]
#[derive(Debug)]
struct Entry {
    base_addr: u64,
    length: u64,
    r#type: u32,
    acpi_attributes: u32,
}

pub fn init(memory_map: u32) -> Result<(), ()> {
    unsafe {
        let entries = &*(memory_map as *const [Entry; 20]);

        let mut largest_entry: Option<&Entry> = None;
        for entry in entries.iter() {
            //            if entry.r#type != 0 {
            //                crate::println!("{:X?}", entry);
            //            }
            if entry.r#type != 1 {
                continue;
            }
            if let Some(current_largest) = largest_entry {
                if entry.length > current_largest.length {
                    largest_entry = Some(entry)
                }
            } else {
                largest_entry = Some(entry)
            }
        }

        match largest_entry {
            Some(entry) => {
                *GLOBAL_ALLOCATOR.arena.get_mut() = entry.base_addr as *mut u8;
                GLOBAL_ALLOCATOR
                    .remaining
                    .store(entry.length as usize, SeqCst);
                Ok(())
            }
            None => Err(()),
        }
    }
}
