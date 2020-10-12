#![feature(panic_info_message)]
#![no_std]

extern crate rlibc;
extern crate multiboot2;
extern crate rangeset;

use rangeset::{Range, RangeSet};

#[macro_use] mod vga_buffer;
mod panic;
mod memory;

#[no_mangle]
fn kernel_entry(multiboot_address: usize) -> ! {
    {
        // Get the lock for the vga buffer and the lock with unlock 
        // when the variable goes out of this scope
        let mut writer = vga_buffer::WRITER.lock();

        // Clear the buffer 
        writer.clear();
    }

    println!("Welcome to NanoOS v0.01");

    // Load the multiboot infomation
    let boot_info = unsafe { multiboot2::load(multiboot_address) };

    if let Some(tag) = boot_info.command_line_tag() {
        let cmd_line = tag.command_line();
        println!("Command Line: {}", cmd_line);
    }

    // Get the memory map from the boot info
    let memory_map = boot_info.memory_map_tag()
        .expect("Failed to retrive the memory map");

    // Construct a rangeset of the physical memory 
    // from the memory map the bootloader gives us
    let mut physical_memory = RangeSet::new();

    // Loop through all the memory areas and and them 
    // to the total physical memory range
    for area in memory_map.memory_areas() {
        // Insert the range
        physical_memory.insert(Range {
            start: area.start_address(),
            end: area.end_address().checked_sub(1).unwrap(),
        });
    }

    // Remove the first 1 MiB becuase there is stuff there we use 
    // or we might use, so to be sure we just remove the first MiB 
    physical_memory.remove(Range {
        start: 0,
        end: 1 * 1024 * 1024 - 1
    });

    memory::init(physical_memory);

    loop {}
}
