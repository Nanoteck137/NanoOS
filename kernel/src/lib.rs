#![no_std]

use core::panic::PanicInfo;

fn kernel_entry() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
