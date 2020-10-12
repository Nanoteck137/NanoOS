use rangeset::{Range, RangeSet};

// TODO(patrik):
//   - Frame Alloctor (Framework, because we can just use RangeSet to 
//     allocate from physical memory for now
//   - Page Tables
//   - Kernel Heap 
//   - Global Allocator (so we can use the core::alloc stuff)

pub fn init(physical_memory: RangeSet) {
    println!("Total Detected Memory: {}MiB", 
             physical_memory.sum().unwrap() / 1024 / 1024);
    println!("Entries: {:#x?}", physical_memory.entries());
}
