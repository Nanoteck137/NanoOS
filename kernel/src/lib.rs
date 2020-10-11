#![feature(panic_info_message)]
#![no_std]

extern crate rlibc;

use core::panic::PanicInfo;

#[macro_use] mod vga_buffer;
mod panic;

#[no_mangle]
fn kernel_entry() -> ! {
    {
        let mut writer = vga_buffer::WRITER.lock();
        writer.clear();
    }
    println!("Hello World from the kernel");
    panic!("Test panic");

    loop {}
}
