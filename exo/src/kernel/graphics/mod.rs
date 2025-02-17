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

//! Exokernel graphics API main module.

use crate::kernel::multiboot::MultibootInfo;

/// VESA framebuffer struct.
#[derive(Debug, Clone, Copy, Default)]
pub struct Framebuffer {
    /// Framebuffer physical address.
    pub addr: u64,
    /// Number of bytes in a single row of the framebuffer.
    pub pitch: u32,
    /// Y-resolution.
    pub width: u32,
    /// X-resolution.
    pub height: u32,
    /// Bytes per pixel.
    pub bpp: u8,
}

impl Framebuffer {
    /// Construct new Framebuffer object.
    ///
    /// # Parameters
    /// - `boot_info` - given multiboot info structure.
    ///
    /// # Returns
    /// New Framebuffer object.
    pub fn new(boot_info: &MultibootInfo) -> Framebuffer {
        Framebuffer {
            addr:   boot_info.framebuffer_addr,
            pitch:  boot_info.framebuffer_pitch,
            width:  boot_info.framebuffer_width,
            height: boot_info.framebuffer_height,
            bpp:    boot_info.framebuffer_bpp,
        }
    }
}