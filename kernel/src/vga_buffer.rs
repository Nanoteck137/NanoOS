use spin::Mutex;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_ADDRESS: usize = 0xb8000;

pub static WRITER: Mutex<VGAWriter> = Mutex::new(VGAWriter{
    x: 0,
    y: 0,
    address: BUFFER_ADDRESS
});

pub struct VGAWriter {
    x: u32,
    y: u32,
    address: usize
}

impl VGAWriter {
    
    // Clear the vga buffer
    // TODO(patrik): Support to pick the background color of the clear
    pub fn clear(&mut self) {
        for offset in 0..(BUFFER_WIDTH * BUFFER_HEIGHT) {
            let address = self.address + offset * 2; 

            unsafe {
                core::ptr::write_volatile(address as *mut u16, 0x0000);
            }
        }

        self.x = 0;
        self.y = 0;
    }

    // Line 1
    // Line 2
    // Line 3
    // Line 4
    // Line 5
    // Line 6

    unsafe fn scroll_up(&mut self) {
        for y in 0..BUFFER_HEIGHT - 1 {
            let current_row = y;
            let next_row = y + 1;

            let src = next_row * BUFFER_WIDTH * 2;
            let dst = current_row * BUFFER_WIDTH * 2;

            let src = self.address + src;
            let dst = self.address + dst;

            core::ptr::copy_nonoverlapping(src as *const u16,
                                           dst as *mut u16,
                                           BUFFER_WIDTH);
        }

        let y = BUFFER_HEIGHT - 1;
        // TODO(patrik): Should replace with a clear row function and support
        // other colors
        for x in 0..BUFFER_WIDTH {
            let offset = (x + (y * BUFFER_WIDTH)) * 2;
            let address = self.address + offset; 

            let color = 0x00;
            let character = b' ';
            let entry = (color as u16) << 8 | character as u16;
            core::ptr::write_volatile(address as *mut u16, entry);
        }
    }

    // Function to print a single character to the screen 
    // with the x and y cordinates
    unsafe fn write_byte(&mut self, character: u8) {
        match character {
            b'\n' => {
                self.y += 1;
                self.x = 0;

                // Check if the y is over the height then we should scroll the
                // buffer
                if self.y as usize >= BUFFER_HEIGHT {
                    // Scroll the buffer up
                    self.scroll_up();
                    // Reset the y 
                    self.y = BUFFER_HEIGHT as u32 - 1;
                }
            }

            _ => {
                // Cast the cordinates to a usize
                let x = self.x as usize;
                let y = self.y as usize;

                // Calculate the offset inside the VGA buffer 
                let offset = (x + (y * BUFFER_WIDTH)) * 2;
                // Add the offset to the address of the buffer
                let address = self.address + offset;

                // The color of the character
                // TODO(patrik): Let the user to pick the color
                let color = 0x0f;
                // The entry we set in the buffer
                let entry = (color as u16) << 8 | (character as u16);

                // Write the entry to the address we calculated
                core::ptr::write_volatile(address as *mut u16, entry);

                // Increment the x cordinate
                self.x += 1;
            }
        }
    }
}

// Implement the fmt Write so we can use the write! macro and 
// so we can implement the print macro
impl core::fmt::Write for VGAWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Loop through all the bytes in the string and write 
        // them to the buffer
        for byte in s.bytes() {
            unsafe {
                self.write_byte(byte)
            }
        }

        Ok(())
    }
}

// Print the format arguments 
pub fn print(args: core::fmt::Arguments) { 
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga_buffer::print(format_args!($($arg)*));
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
