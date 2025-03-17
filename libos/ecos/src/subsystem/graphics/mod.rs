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

//! Graphics module. Contains declarations for RGB colors and
//! other graphics related functions.

pub use eciton_sdk::vbe::Framebuffer;
use super::{Subsystem, SubsystemResult};
use crate::{api::exo, printk};

pub mod terminal;
pub mod font;

/// RGB color type.
pub type Rgb = u32;

/// Make RGB color from red, green and blue components.
///
/// # Usage
/// ```
/// let white_color: Rgb = rgb!(255, 255, 255);
/// ```
#[macro_export]
macro_rules! rgb {
    ($red:expr, $green:expr, $blue:expr) => {
        (($red as u32) << 16) | (($green as u32) << 8) | ($blue as u32)
    };
}

/// Standard color enumeration.
#[repr(u32)]
pub enum Color {
    White = rgb!(0xFF, 0xFF, 0xFF),
    Black = rgb!(0x00, 0x00, 0x00),
    Red   = rgb!(0xFF, 0x00, 0x00),
    Green = rgb!(0x00, 0xFF, 0x00),
    Blue  = rgb!(0x00, 0x00, 0xFF),
    Gray  = rgb!(0xBF, 0xBF, 0xBF),
}

/// Graphics subsystem struct.
#[derive(Default, Clone, Copy)]
pub struct GraphicsSub {
    /// VESA framebuffer struct.
    pub fb: Framebuffer,
}

impl Subsystem for GraphicsSub {
    /// Initialize graphics subsystem.
    ///
    /// #Returns
    /// - `Ok`       - in case of success.
    /// - `Err(msg)` - error message otherwise.
    fn init(&self) -> SubsystemResult {
        // Do nothing.
        Ok(())
    }

    /// Run graphics subsystem.
    ///
    /// #Returns
    /// - `Ok`       - in case of success.
    /// - `Err(msg)` - error message otherwise.
    fn run(&mut self) -> SubsystemResult {
        // Get framebuffer info.
        match exo::getfb() {
            Ok(fb) => self.fb = fb,
            Err(_) => {},
        }

        // Initialize kernel log functions.
        printk::init(*self);
        Ok(())
    }

    /// Shutdown graphics subsystem.
    ///
    /// #Returns
    /// - `Ok`       - in case of success.
    /// - `Err(msg)` - error message otherwise.
    fn exit(&self) -> SubsystemResult {
        // Do nothing.
        Ok(())
    }

    /// Get subsystem name.
    ///
    /// #Returns
    /// - Subsystem name in string representation.
    fn name(&self) -> &'static str {
        "Graphics Subsystem"
    }
}

impl GraphicsSub {
    /// Put pixel on the screen.
    ///
    /// # Parameters
    /// - `x`     - given x-coordinate of pixel.
    /// - `y`     - given y-coordinate of pixel.
    /// - `color` - given RGB color of pixel.
    #[inline]
    pub fn put_pixel(&self, x: u32, y: u32, color: Rgb) {
        if x < self.fb.width && y < self.fb.height {
            unsafe {
                let buffer = self.fb.addr as *mut u32;
                *buffer.offset((y * self.fb.width + x) as isize) = color;
            }
        }
    }

    /// Draw font character on the screen.
    ///
    /// # Parameters
    /// - `c`     - given character to print.
    /// - `x`     - given x pixel position.
    /// - `y`     - given y pixel position.
    /// - `fg`    - given foreground color.
    /// - `bg`    - given background color.
    /// - `is_bg` - given param determine whether to display the `bg`.
    pub fn draw_char(&self, c: char, x: u32, y: u32, fg: Rgb, bg: Rgb, is_bg: bool) {
        static MASK: [u8; 8] = [ 128, 64, 32, 16, 8, 4, 2, 1 ];
        let font_ptr         = font::FONT.as_ptr();
        let glyph: *const u8 = unsafe { font_ptr.add(c as usize * 16) };

        let mut pixel: u8;

        for cy in 0..font::CHAR_HEIGHT {
            for cx in 0..font::CHAR_WIDTH {
                pixel = unsafe { *glyph.add(cy as usize) };

                if pixel & MASK[cx as usize] != 0 {
                    self.put_pixel(x + cx, y + cy, fg);
                }
                else if is_bg {
                    self.put_pixel(x + cx, y + cy, bg);
                }
            }
        }
    }

    /// Fill screen with specific color.
    ///
    /// # Parameters
    /// - `color` - given color to fill with.
    pub fn fill_screen(&self, color: Rgb) {
        for y in 0..self.fb.height {
            for x in 0..self.fb.width {
                self.put_pixel(x, y, color);
            }
        }
    }
}