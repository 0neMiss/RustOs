#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//repr(C) allows us to enforce enum order
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
// repr transparent is support for some type recasting that happens between C and rust(i think)
// https://doc.rust-lang.org/1.26.2/unstable-book/language-features/repr-transparent.html#enter-reprtransparent
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer { 
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer
}

impl Writer {
    pub fn write_string(&mut self, s: &str){
        for byte in s.bytes(){
            match byte {
                // if the bite is an ASCII character then write it
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Otherwise write █
                _ => self.write_byte(0xfe),
            }
        }
    }
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            // if the byte is of the type new line, create  new line
            b'\n' => self.new_line(),
            byte => {
                // if the current width of our text is too wide, overflow this char to next line
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line()
                }

                let row = BUFFER_HEIGHT -1;
                let col = self.column_position;

                let color_code = self.color_code;
                // inside the buffer, create a entry for the character we are writing to the screen.
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code
                };
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {}
}
pub fn print_something(){
    // create a writer that points to the VGA buffer at 0xb8000
    /* . The syntax for this might seem a bit strange: 
    First, we cast the integer 0xb8000 as a mutable raw pointer. 
    Then we convert it to a mutable reference by dereferencing it (through *) 
    and immediately borrowing it again (through &mut). */

    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer)},
    };
    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}