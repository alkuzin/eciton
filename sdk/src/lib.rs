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

//! EcitonSDK main module.

#![no_std]                      // Do not use the standard library.
#![no_main]                     // Do not use the standard main function.
#![allow(clippy::empty_loop)]   // Ignore empty loop.
#![allow(dead_code)]            // Allow unused values.

/// VESA framebuffer struct.
#[derive(Debug, Default, Clone, Copy)]
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