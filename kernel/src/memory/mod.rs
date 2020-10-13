use rangeset::{Range, RangeSet};

struct VirtualAddress(u64);
struct PhysicalAddress(u64);

struct Frame {
}

trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame) -> Option<()>;
}

impl FrameAllocator for RangeSet {
    fn allocate_frame(&mut self) -> Option<Frame> {
        println!("Called allocate");
        None
    }

    fn deallocate_frame(&mut self, frame: Frame) -> Option<()> {
        None
    }
}

// TODO(patrik):
//   - Frame Alloctor (Framework, because we can just use RangeSet to 
//     allocate from physical memory for now but in the future 
//     we might want something else
//   - Page Tables
//   - Kernel Heap 
//   - Global Allocator (so we can use the core::alloc stuff)

pub fn init(physical_memory: &mut RangeSet) {
    println!("Total Detected Memory: {}MiB", 
             physical_memory.sum().unwrap() / 1024 / 1024);
    println!("Entries: {:#x?}", physical_memory.entries());

    physical_memory.allocate_frame().unwrap();
}
