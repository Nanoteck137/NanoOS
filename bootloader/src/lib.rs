#![feature(lang_items, panic_info_message)]

#![no_std]

extern crate spin;

use core::panic::PanicInfo;

use spin::Mutex;

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

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO(patrik): Should we disable interupts?
    println!("---------- BOOTLOADER PANIC ----------");
    
    if let Some(msg) = info.message() {
        println!("Message: {}", msg);
    }

    if let Some(loc) = info.location() {
        println!("Location: {}:{}:{}", 
                 loc.file(), loc.line(), loc.column());
    }

    println!("--------------------------------------");

    // TODO(patrik): Replace with a halt instruction
    loop {}
}
