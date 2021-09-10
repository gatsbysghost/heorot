use volatile::Volatile;
use core::fmt;

/// Static values for colors in a C-style struct
#[allow(dead_code)] // Don't throw compiler errors for unused items
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // Store each color's byte value in an 8-bit number
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

/// New ColorCode type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // We'll be using this throughout to ensure that the data layouts on derived funcs match the underlying u8 etc.
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// New ScreenChar type; first 8 bits are ascii code; next 7 bits are color; final bit is blink
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // We need this because we care about byte ordering but Rust doesn't order fields strictly
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// New Buffer type (for the text buffer)
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// New Writer type, this writes to the screen.
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer, // 'static specifies that the reference is valid for the whole program's run time
}

/// Allow us to write a single ASCII byte
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), // If the character given is \n, then call the new_line method
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line(); // If the current line is full, insert a newline
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                // Write a new ScreenChar at the current position
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {/* TODO */}
}

/// Function to write a string
impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range; print a box
                _ => self.write_byte(0xfe),
            }

        }
    }
}

/// Test function to print a Hello World! (remove later)
pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("World!");
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}

/// Add support for Rust's core write methods & formatting macros
impl fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write_string(s);
    Ok(())
  }
}
