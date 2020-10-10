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

    let mut range_set = RangeSet::new();

    unsafe {
        let entry_ptr = 0x0500 as *const MemoryMapEntry;
        let mut entry = entry_ptr.offset(0);
        loop {
            let e = *entry;
            if e.typ == 0 {
                break;
            }

            // println!("Entry: {:#x?}", e);
            if e.typ == 1 {
                range_set.insert(Range { 
                    start: e.address, 
                    end: e.address.checked_add(e.length - 1).unwrap()
                });
            }

            entry = entry.offset(1);
        }
    }

    range_set.remove(Range {
        start: 0,
        end: 1 * 1024 * 1024 - 1
    });

    println!("Entries: {:#x?}", range_set.entries()); 

    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

