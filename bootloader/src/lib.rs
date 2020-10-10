#![feature(lang_items, panic_info_message)]

#![no_std]

extern crate spin;

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

    unsafe {
        let entry_ptr = 0x0500 as *const MemoryMapEntry;
        let mut entry = entry_ptr.offset(0);
        let mut offset = 0;
        loop {
            if (*entry).typ == 0 {
                break;
            }

            println!("Entry: {:#x?}", *entry);

            entry = entry.offset(1);
        }
    }

    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

