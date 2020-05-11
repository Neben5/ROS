use core::fmt; // string formatting
use lazy_static::lazy_static; // global static objects, writer
use spin::Mutex; // prevents races for writer
use volatile::Volatile; // prevents compiler from ignoring vga register writes, since they're unsafe

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // inherited?
#[repr(u8)] // u8
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)] // has these, sort of inherited
#[repr(transparent)]
struct ColorCode(u8); // ColorCode is a u8

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        // constructor (Color, Color)
        ColorCode((background as u8) << 4 | (foreground as u8)) // color byte is (a,a,a,a,b,b,b,b), a for bg, b for fg
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // use the c ordering of memory in structs, makes sure that the character is before the color
           // key, since this is being pushed directly into the register
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)] // has memory layout of 2d array, not anything else
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT], // char: array of ScreenChar's, [bufferwidth][bufferheight]
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), // on \n, newline
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line(); // on end of row, newline
                }

                let row = BUFFER_HEIGHT - 1; //always write to bottom row
                let col = self.column_position; // write to current column pos

                let color_code = self.color_code; // use color_code
                self.buffer.chars[row][col].write(ScreenChar {
                    // at (x,y)
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1; // next column pos
            }
        }
    }

    pub fn new_line(&mut self) {
        self.column_position = 0;
        for row in 1..BUFFER_HEIGHT {
            // start at 1, dont shift top up to missing register
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1); // clear bottom row, since not getting overwritten
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                b' '..=b'~' | b'\n' => self.write_byte(byte), // if byte matches printable char or newline
                _ => self.write_byte(0xfe),                   // not a printable byte, print ■
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer { // globally accessible writer using a spinning mutex
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }, // writing to this register is safe
                                                           // maps register to buffer struct, abstracts
                                                           // unsafeness away
    });
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*) =>{
        $crate::print!("{}\n",format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print(format_args!($($arg)*));
    };

}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap(); // locks writer, passes args to write_fmt, which stringifies and passes to write_string, etc
}
