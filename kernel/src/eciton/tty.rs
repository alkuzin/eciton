// Eciton - experimental exokernel.
// Copyright (C) 2025 Alexander (@alkuzin).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! TODO:

use crate::eciton::gfx;
use core::fmt;

use gfx::Rgb;

use super::vesa::Vesa;

const TAB_WIDTH: u32 = 4;

#[derive(Default)]
pub struct Terminal {
    vesa:   Vesa,
    height: i32,    // Screen height.
    width:  i32,    // Screen width.
    x_pos:  i32,    // X position of the cursor.
    y_pos:  i32,    // Y position of the cursor.
    fg:     Rgb,    // Foreground color.
    bg:     Rgb,    // Background color.
}

impl Terminal {
    /// Scroll screen.
    fn scroll(&self) {

    }

    pub fn set(&mut self, vesa: Vesa) {
        self.vesa  = vesa;
        self.x_pos =  0;
        self.y_pos =  0;
        self.fg =     gfx::Color::White as u32;
        self.bg =     gfx::Color::Black as u32;
        self.height = self.vesa.fb.height as i32;
        self.width =  self.vesa.fb.width as i32;
    }

    /// Clear screen.
    pub fn clear(&self) {
        self.vesa.fill_screen(self.bg);
    }

    /// Print colored character on screen.
    ///
    /// # Parameters
    ///
    /// `c`  - given character to print.
    /// `fg` - given foreground color.
    /// `bg` - given background color.
    pub fn putc(&mut self, c: char, fg: Rgb, bg: Rgb) {
        if self.x_pos >= self.width {
            self.x_pos = 0;
            self.y_pos = gfx::FONT_CHAR_HEIGHT as i32;
        }

        match c {
            '\n' => {
                self.y_pos += gfx::FONT_CHAR_HEIGHT as i32;
                self.x_pos = 0;
            },
            '\t' => {
                for _ in 0..TAB_WIDTH {
                    self.vesa.draw_char(c, self.x_pos as u32, self.y_pos as u32, fg, bg, true);
                    self.x_pos += gfx::FONT_CHAR_WIDTH as i32;
                }
            },
            _ => {
                if c == ' ' || c.is_ascii_graphic() {
                    self.vesa.draw_char(c, self.x_pos as u32, self.y_pos as u32, fg, bg, true);
                    self.x_pos += gfx::FONT_CHAR_WIDTH as i32;
                }
            }
        }

        if self.y_pos >= self.height {
            let tmp  = self.y_pos - self.height;
            let rows = tmp / gfx::FONT_CHAR_HEIGHT as i32 + 1;
            self.scroll();
            self.y_pos -= rows * gfx::FONT_CHAR_HEIGHT as i32;
        }
    }
}

impl fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.putc(c, self.fg, self.bg);
        }
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Terminal> = Mutex::new(Terminal::default());
}

#[macro_export]
macro_rules! putk {
    ($($arg:tt)*) => ($crate::eciton::tty::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! printk {
    () => ($crate::putk!("\n"));
    ($($arg:tt)*) => ($crate::putk!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}