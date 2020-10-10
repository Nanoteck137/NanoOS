#![feature(lang_items, panic_info_message)]

#![no_std]

extern crate spin;

mod panic;
#[macro_use] mod vga_buffer;

#[no_mangle]
#[link_section = ".start"]
extern fn entry() -> ! {
    {
        let mut writer = vga_buffer::WRITER.lock();
        writer.clear();
    }

    println!("Welcome to NanoOS Bootloader v0.1");

    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

