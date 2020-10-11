#![no_std]

extern crate rlibc;

use core::panic::PanicInfo;

#[no_mangle]
fn kernel_entry() -> ! {
    let screen = 0xb8000 as *mut u16;
    unsafe {
        *screen.offset(0) = 0x0f41;
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
