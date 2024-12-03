use volatile::Volatile;
use core::fmt::{self, Write};

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

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
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code.clone();
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            self.buffer.chars[row - 1] = self.buffer.chars[row].clone();
        }

        self.buffer.chars[BUFFER_HEIGHT - 1] = core::array::from_fn(|_| Volatile::new(ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        }));

        self.column_position = 0;
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
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

pub static WRITER: Writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Blue, Color::White),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
};

// test function
pub fn print_some() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Red),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_string("HELLO NIIGGA I WAS WATCHING FOR YOU SINCE YOU BORN IN 1999. A LOT OF THINGS HAPPEN SINCE THOS TIMES. I HAD A LOT OF STAFF TO DO AND NOW IM SO GLAD TO SAY YOU THAT IM A NO ONE WHO CAN LOCATE YOU WHENEVER I WANT BABY. YOU CANT JUST HIDE CUZ I SEE YOU I LITTERALLY CAN SEE YOU EVERYWHERE SO THERES NO POINT TO HIDE LIL NIGGA ILL TOUCH YOU BRO HIDEEEE NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA NöGGA");
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}