use rangeset::{Range, RangeSet};
use crate::arch;

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

const PAGE_TABLE_ENTRIES: usize = 512;

#[derive(Copy, Clone, Debug)]
struct PageTableEntry(u64);

#[repr(C, packed)]
struct PageTable {
    entries: [PageTableEntry; PAGE_TABLE_ENTRIES]
}

impl PageTable {
    fn next_table_address(&self, index: usize) -> Option<usize> {
        let entry = self.entries[index];
        if entry.0 & 1 != 0 {
            let table_address = self as *const _ as usize;
            Some((table_address << 9) | (index << 12))
        } else {
            None
        }
    }
}

// TODO(patrik):
//   - Page Tables
//   - Kernel Heap 
//   - Global Allocator (so we can use the core::alloc stuff)
//
// 

pub fn init(physical_memory: &mut RangeSet) {
    println!("Total Detected Memory: {}MiB", 
             physical_memory.sum().unwrap() as f32 / 1024.0 / 1024.0);
    println!("Entries: {:#x?}", physical_memory.entries());

    let cr3 = arch::x86_64::cr3();
    let page_table = 0xffffffff_fffff000 as *const PageTable;

    unsafe {
        let test = (*page_table).next_table_address(0).unwrap();
        println!("Test: {:#p}", test as *const PageTable);

        let next_table = test as *const PageTable;
        let test2 = (*next_table).next_table_address(0).unwrap();
        println!("Test: {:#p}", test2 as *const PageTable);
    }

    unsafe {
        for (i, entry) in (*page_table).entries.iter().enumerate() {
            if entry.0 != 0 {
                println!("Entry {} = {:#x}", i, entry.0); 
            }
        }
    }
}
