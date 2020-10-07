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
    loop {}
}
