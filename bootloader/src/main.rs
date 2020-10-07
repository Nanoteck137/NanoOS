#![feature(lang_items)]

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[lang = "eh_personality"] extern fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern fn entry() -> ! {
    let screen = 0xb8000 as *mut u16;
    unsafe {
        *screen.offset(0) = 0x0f43;
    }
    loop {}
}
