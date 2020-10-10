#![feature(lang_items)]

#![no_std]

use core::panic::PanicInfo;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_ADDRESS: usize = 0xb8000;

fn write_char(character: u8, x: u32, y: u32) {
    let x = x as usize;
    let y = y as usize;
    let offset = (x + (y * BUFFER_WIDTH)) * 2;
    let address = BUFFER_ADDRESS + offset;

    let color = 0x0f;
    let character = (color as u16) << 8 | (character as u16);

    unsafe {
        core::ptr::write_volatile(address as *mut u16, character);
    }
}

#[no_mangle]
#[link_section = ".start"]
extern fn entry() -> ! {
    write_char(b'c', 1, 2);
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
