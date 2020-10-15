use spin::Mutex;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_ADDRESS: usize = 0xb8000;

pub static WRITER: Mutex<VGAWriter> = Mutex::new(VGAWriter{
    x: 0,
    y: 0,
    address: BUFFER_ADDRESS,
    foreground_color: Color::White,
    background_color: Color::Black,
    clear_color: Color::Black
});

#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

fn encode_color(foreground: Color, background: Color) -> u8 {
    (background as u8) << 4 | (foreground as u8)
}

pub struct VGAWriter {
    x: u32,
    y: u32,
    address: usize,

    foreground_color: Color,
    background_color: Color,
    clear_color: Color
}

impl VGAWriter {
    
    // Clear the vga buffer
    // TODO(patrik): Support to pick the background color of the clear
    pub fn clear(&mut self, clear_color: Color) {
        for offset in 0..(BUFFER_WIDTH * BUFFER_HEIGHT) {
            let address = self.address + offset * 2; 

            let color = encode_color(Color::White, clear_color);
            let character = b' ';
            let entry = (color as u16) << 8 | (character as u16);

            unsafe {
                core::ptr::write_volatile(address as *mut u16, entry);
            }
        }

        self.x = 0;
        self.y = 0;
        self.clear_color = clear_color;
    }

    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.foreground_color = foreground;
        self.background_color = background;
    }

    fn scroll_up(&mut self) {
        for y in 0..BUFFER_HEIGHT - 1 {
            let current_row = y;
            let next_row = y + 1;

            let src = next_row * BUFFER_WIDTH * 2;
            let dst = current_row * BUFFER_WIDTH * 2;

            let src = self.address + src;
            let dst = self.address + dst;

            unsafe {
                core::ptr::copy_nonoverlapping(src as *const u16,
                                               dst as *mut u16,
                                               BUFFER_WIDTH);
            }
        }

        let y = BUFFER_HEIGHT - 1;
        // TODO(patrik): Should replace with a clear row function and support
        // other colors
        for x in 0..BUFFER_WIDTH {
            // Calculate the offset
            let offset = (x + (y * BUFFER_WIDTH)) * 2;
            // Add the offset to the base address for the buffer
            let address = self.address + offset; 

            // Construct the entry for the buffer
            let color = encode_color(Color::White, self.clear_color);
            let character = b' ';
            let entry = (color as u16) << 8 | character as u16;

            unsafe {
                // Write the entry to the buffer at the address calculated
                core::ptr::write_volatile(address as *mut u16, entry);
            }
        }
    }

    // Function to print a single character to the screen 
    // with the x and y cordinates
    fn write_byte(&mut self, character: u8) {
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
                let color = 
                    encode_color(self.foreground_color, self.background_color);
                // The entry we set in the buffer
                let entry = (color as u16) << 8 | (character as u16);

                unsafe {
                    // Write the entry to the address we calculated
                    core::ptr::write_volatile(address as *mut u16, entry);
                }

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
            self.write_byte(byte)
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
