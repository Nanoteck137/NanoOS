use rangeset::{Range, RangeSet};
use crate::arch;

const PAGE_PRESENT:       u64 = 1 <<  0;
const PAGE_WRITE:         u64 = 1 <<  1;
const PAGE_USER:          u64 = 1 <<  2;
const PAGE_WRITE_THROUGH: u64 = 1 <<  3;
const PAGE_NO_CACHE:      u64 = 1 <<  4;
const PAGE_ACCESSED:      u64 = 1 <<  5;
const PAGE_DIRTY:         u64 = 1 <<  6;
const PAGE_HUGE:          u64 = 1 <<  7;
const PAGE_GLOBAL:        u64 = 1 <<  8;
const PAGE_NXE:           u64 = 1 << 63;

#[derive(Copy, Clone, Debug)]
struct VirtualAddress(u64);

#[derive(Copy, Clone, Debug)]
struct PhysicalAddress(u64);

#[derive(Copy, Clone, Debug)]
struct PhysicalFrame(u64);

impl PhysicalFrame {
    fn containing_address(address: PhysicalAddress) -> Self {
        // TODO(patrik): Change the 4096 to a constant "PAGE SIZE"
        Self(address.0 / 4096)
    }
}

#[derive(Copy, Clone, Debug)]
struct Page(u64);

impl Page {
    fn containing_address(address: VirtualAddress) -> Self {
        assert!(address.0 < 0x0000_8000_0000_0000 ||
                address.0 >= 0xffff_8000_0000_0000,
                "Invalid Virtual Address: {:?}", address);

        // TODO(patrik): Change the 4096 to a constant "PAGE SIZE"
        Page(address.0 / 4096)
    }

    fn p4_index(&self) -> usize {
        ((self.0 >> 27) & 0x1ff) as usize
    }

    fn p3_index(&self) -> usize {
        ((self.0 >> 18) & 0x1ff) as usize
    }

    fn p2_index(&self) -> usize {
        ((self.0 >> 9) & 0x1ff) as usize
    }

    fn p1_index(&self) -> usize {
        ((self.0 >> 0) & 0x1ff) as usize
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

impl PageTableEntry {
    fn pointed_frame(&self) -> Option<PhysicalFrame> {
        if self.0 & PAGE_PRESENT != 0 {
            Some(PhysicalFrame::containing_address(
                PhysicalAddress(self.0 & 0x000fffff_fffff000)
            ))
        } else {
            None
        }
    }
}

#[repr(C, packed)]
struct PageTable {
    entries: [PageTableEntry; PAGE_TABLE_ENTRIES]
}

impl PageTable {
    fn next_table_address(&self, index: usize) -> Option<usize> {
        let entry = self.entries[index];
        if entry.0 & PAGE_PRESENT != 0 {
            let table_address = self as *const _ as usize;
            Some((table_address << 9) | (index << 12))
        } else {
            None
        }
    }

    fn next_table<'a>(&'a self, index: usize) -> Option<&'a PageTable> {
        self.next_table_address(index)
            .map(|x| unsafe { &*(x as *const _) })
    }

    fn next_table_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut PageTable> {
        self.next_table_address(index)
            .map(|x| unsafe { &mut *(x as *mut _) })
    }
}

const P4: *const PageTable = 0xffffffff_fffff000 as *const _;

fn translate_page(page: Page) -> Option<PhysicalFrame> {
    let p3 = unsafe { &*P4 }.next_table(page.p4_index());

    let huge_page = || {
        p3.and_then(|p3| {
            if let Some(p2) = p3.next_table(page.p3_index()) {
                let p2_entry = &p2.entries[page.p2_index()];
                if let Some(start_frame) = p2_entry.pointed_frame() {
                    if p2_entry.0 & PAGE_HUGE != 0 {
                        assert!(start_frame.0 % 512 == 0);

                        let frame = 
                            PhysicalFrame(start_frame.0 + 
                                          page.p1_index() as u64);

                        return Some(frame);
                    }
                }
            }

            None
        })
    };

    let test = 
        unsafe {
            p3.and_then(|p3| p3.next_table(page.p3_index()))
            .and_then(|p2| p2.next_table(page.p2_index()))
            .and_then(|p1| p1.entries[page.p1_index()].pointed_frame())
            .or_else(huge_page)
        };
    println!("Page: {:?}", page);
    println!("p1_index: {}", page.p1_index());
    println!("Test: {:#x?}", test);

    None
}

// TODO(patrik):
//   - Page Tables
//   - Kernel Heap 
//   - Global Allocator (so we can use the core::alloc stuff)

fn print_table_entries(table: &PageTable) {
    unsafe {
        for (i, entry) in table.entries.iter().enumerate() {
            if entry.0 != 0 {
                println!("Entry {} = {:#x}", i, entry.0); 
            }
        }
    }
}

pub fn init(physical_memory: &mut RangeSet) {
    println!("Total Detected Memory: {}MiB", 
             physical_memory.sum().unwrap() as f32 / 1024.0 / 1024.0);
    println!("Entries: {:#x?}", physical_memory.entries());

    let cr3 = arch::x86_64::cr3();
    let page_table = unsafe { &*P4 };

    let page = Page::containing_address(VirtualAddress(0x0000));
    translate_page(page);

    println!("P4");
    print_table_entries(&page_table);

    let p3_table = page_table.next_table(0).unwrap();
    println!("P3");
    print_table_entries(&p3_table);
}
