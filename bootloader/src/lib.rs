#![feature(lang_items)]

#![no_std]

extern crate spin;

use core::panic::PanicInfo;

use spin::Mutex;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_ADDRESS: usize = 0xb8000;

struct VGAWriter {
    x: u32,
    y: u32,
    address: usize
}

impl VGAWriter {
    // Function to print a single character to the screen 
    // with the x and y cordinates
    fn write_byte(&mut self, character: u8) {
        let x = self.x as usize;
        let y = self.y as usize;

        let offset = (x + (y * BUFFER_WIDTH)) * 2;
        let address = self.address + offset;

        let color = 0x0f;
        let character = (color as u16) << 8 | (character as u16);

        unsafe {
            core::ptr::write_volatile(address as *mut u16, character);
        }

        self.x += 1;
    }
}

impl core::fmt::Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
          self.write_byte(byte)
        }
        Ok(())
    }
}

fn print(args: core::fmt::Arguments) { 
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

static WRITER: Mutex<VGAWriter> = Mutex::new(VGAWriter{
    x: 0,
    y: 0,
    address: BUFFER_ADDRESS
});

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::print(format_args!($($arg)*));
    });
}

#[no_mangle]
#[link_section = ".start"]
extern fn entry() -> ! {
    print!("{}", { print!("inner"); "outer" });

    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
