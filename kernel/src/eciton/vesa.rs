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

//! Contains Video Electronics Standards Association (VESA) driver declaration.

use crate::eciton::{gfx::{self, FONT_CHAR_WIDTH}, MultibootInfo};
use gfx::Rgb;


#[derive(Debug, Clone, Copy)]
pub struct Framebuffer
{
    pub addr:   u64,
    pub pitch:  u32,
    pub width:  u32,
    pub height: u32,
    pub bpp:    u8,
}

impl Framebuffer
{
    pub fn new(boot_info: &MultibootInfo) -> Framebuffer
    {
        Framebuffer {
            addr:   boot_info.framebuffer_addr,
            pitch:  boot_info.framebuffer_pitch,
            width:  boot_info.framebuffer_width,
            height: boot_info.framebuffer_height,
            bpp:    boot_info.framebuffer_bpp,
        }
    }
}

pub struct Vesa
{
    fb:     Framebuffer,
    buffer: *mut u32,
}

impl Vesa
{
    pub fn new(boot_info: &MultibootInfo) -> Vesa
    {
        let fb     = Framebuffer::new(boot_info);
        let buffer = fb.addr as *mut u32;

        Vesa { fb, buffer }
    }

    #[inline]
    pub fn put_pixel(&self, x: u32, y: u32, color: Rgb)
    {
        if x < self.fb.width && y < self.fb.height {
            unsafe {
                *self.buffer.offset((y * self.fb.width + x) as isize) = color;
            }
        }
    }

    pub fn draw_char(&self, c: char, x: u32, y: u32, fg: Rgb)
    {
        static MASK: [u8; 8] = [ 128, 64, 32, 16, 8, 4, 2, 1 ];

        let font_ptr         = gfx::FONT.as_ptr();
        let glyph: *const u8 = unsafe { font_ptr.add(c as usize * 16) };

        let mut pixel: u8;

        for cy in 0..gfx::FONT_CHAR_HEIGHT {
            for cx in 0..FONT_CHAR_WIDTH {
                pixel = unsafe { *glyph.add(cy as usize) };

                if pixel & MASK[cx as usize] != 0 {
                    self.put_pixel(x + cx, y + cy, fg);
                }
            }
        }
    }
}