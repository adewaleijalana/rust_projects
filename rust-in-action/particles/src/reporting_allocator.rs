use std::{alloc::{GlobalAlloc, System, Layout}, time::Instant};


#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

pub struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = unsafe { System.alloc(layout) };
        let end = Instant::now();

        let time_taken = end - start;
        let bytes_requested = layout.size();

        eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());

        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
    }
}