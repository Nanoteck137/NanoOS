use rangeset::{Range, RangeSet};

#[derive(Copy, Clone, Debug)]
struct VirtualAddress(u64);

#[derive(Copy, Clone, Debug)]
struct PhysicalAddress(u64);

#[derive(Copy, Clone, Debug)]
struct PhysicalFrame(u64);

impl PhysicalFrame {
    fn containing_address(address: PhysicalAddress) -> Self {
        Self(address.0 / 4096)
    }
}

trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysicalFrame>;
    fn deallocate_frame(&mut self, frame: PhysicalFrame) -> Option<()>;
}

impl FrameAllocator for RangeSet {
    fn allocate_frame(&mut self) -> Option<PhysicalFrame> {
        let address = self.allocate(4096, 4096)?;
        let address = PhysicalAddress(address as u64);
        Some(PhysicalFrame::containing_address(address))
    }

    fn deallocate_frame(&mut self, frame: PhysicalFrame) -> Option<()> {
        let start = frame.0 * 4096;
        let end = start.checked_add(4096 - 1)?;

        self.insert(Range {
            start: start,
            end: end
        });

        Some(())
    }
}

// TODO(patrik):
//   - Page Tables
//   - Kernel Heap 
//   - Global Allocator (so we can use the core::alloc stuff)

pub fn init(physical_memory: &mut RangeSet) {
    println!("Total Detected Memory: {}MiB", 
             physical_memory.sum().unwrap() / 1024 / 1024);
    println!("Entries: {:#x?}", physical_memory.entries());
}
