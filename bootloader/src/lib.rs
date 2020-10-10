#![feature(lang_items, panic_info_message)]

#![no_std]

extern crate spin;
extern crate rangeset;

use rangeset::{Range, RangeSet};

#[macro_use] mod vga_buffer;
mod panic;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct MemoryMapEntry {
    address: u64,
    length: u64,
    typ: u32,
    acpi: u32,
}

#[no_mangle]
#[link_section = ".start"]
extern fn entry() -> ! {
    {
        let mut writer = vga_buffer::WRITER.lock();
        writer.clear();
    }

    println!("Welcome to NanoOS Bootloader v0.1");

    // Initialize an empty range set
    let mut range_set = RangeSet::new();

    unsafe {
        // Setup a pointer to the first memory map entry located at 0x0500, 
        // this address is specified in the stage0 code
        let entry_ptr = 0x0500 as *const MemoryMapEntry;
        // Get the first entry
        let mut entry = entry_ptr.offset(0);
        loop {
            // Dereference the pointer
            let e = *entry;
            // Check if this entry is the last, the last entry has the typ = 0
            if e.typ == 0 {
                break;
            }

            // typ = 1 is usable memory so add it to the range set
            if e.typ == 1 {
                // Insert the range
                range_set.insert(Range { 
                    start: e.address, 
                    end: e.address.checked_add(e.length - 1).unwrap()
                });
            }

            // Get the next entry
            entry = entry.offset(1);
        }
    }

    // Remove the first 1 MiB because it has alot of memory map 
    // that could be usefull
    range_set.remove(Range {
        start: 0,
        end: 1 * 1024 * 1024 - 1
    });

    println!("Entries: {:#x?}", range_set.entries()); 

    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

