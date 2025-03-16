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

//! Contains kernel terminal declaration.

use crate::subsystem::graphics::{Rgb, Color, Graphics, font};
use core::ptr::write_bytes;

/// Default tabulation width.
const TAB_WIDTH: u32 = 4;

/// Kernel terminal struct.
#[derive(Default)]
pub struct Terminal {
    /// Graphics handler.
    gfx: Graphics,
    /// Screen height.
    height: i32,
    /// Screen width.
    width: i32,
    /// X position of the cursor.
    x_pos: i32,
    /// Y position of the cursor.
    y_pos: i32,
    /// Foreground color.
    pub fg: Rgb,
    /// Background color.
    pub bg: Rgb,
}

impl Terminal {
    /// Scroll screen.
    fn scroll(&self) {
        let fb     = self.gfx.fb;
        let size   = fb.height * fb.pitch;
        let buffer = fb.addr as *mut u32;
        let mut pos: u32;

        for i in 0..size {
            pos = i + fb.width * font::CHAR_HEIGHT;
            unsafe {
                *buffer.offset(i as isize) = *buffer.offset(pos as isize);
            }
        }

        pos = size - fb.width * font::CHAR_HEIGHT;

        unsafe {
            write_bytes(buffer.offset(pos as isize), 0, (size - pos) as usize);
        }
    }

    /// Initialize Terminal.
    ///
    /// # Parameters
    /// - `gfx` - graphics handler object.
    pub fn init(&mut self, gfx: Graphics) {
        self.gfx    = gfx;
        self.x_pos  = 0;
        self.y_pos  = 0;
        self.fg     = Color::White as u32;
        self.bg     = Color::Black as u32;
        self.height = self.gfx.fb.height as i32;
        self.width  = self.gfx.fb.width as i32;
    }

    /// Clear screen.
    pub fn clear(&self) {
        self.gfx.fill_screen(self.bg);
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
            self.y_pos = font::CHAR_HEIGHT as i32;
        }

        match c {
            // Handle new line character.
            '\n' => {
                self.y_pos += font::CHAR_HEIGHT as i32;
                self.x_pos = 0;
            },
            // Handle tab character
            '\t' => {
                for _ in 0..TAB_WIDTH {
                    self.gfx.draw_char(
                        c,
                        self.x_pos as u32,
                        self.y_pos as u32,
                        fg,
                        bg,
                        true
                    );

                    self.x_pos += font::CHAR_WIDTH as i32;
                }
            },
            // Handle other characters.
            _ => {
                if c == ' ' || c.is_ascii_graphic() {
                    self.gfx.draw_char(
                        c,
                        self.x_pos as u32,
                        self.y_pos as u32,
                        fg,
                        bg,
                        true
                    );

                    self.x_pos += font::CHAR_WIDTH as i32;
                }
            }
        }

        // Scroll screen.
        if self.y_pos >= self.height {
            let tmp  = self.y_pos - self.height;
            let rows = tmp / font::CHAR_HEIGHT as i32 + 1;
            self.scroll();
            self.y_pos -= rows * font::CHAR_HEIGHT as i32;
        }
    }
}