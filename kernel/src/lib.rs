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

//! Kernel entry point.

#![no_std]                      // Do not use the standard library.
#![no_main]                     // Do not use the standard main function.
#![allow(clippy::empty_loop)]   // Ignore empty loop.
#![allow(dead_code)]            // Allow unused values.

mod eciton;
use eciton::multiboot::{MULTIBOOT_BOOTLOADER_MAGIC, MultibootInfo};

/// Kernel entry point.
///
/// # Parameters
/// - `magic`     - given multiboot magic number.
/// - `boot_info` - given multiboot info structure.
#[unsafe(no_mangle)]
extern "C" fn kmain(magic: u32, boot_info: &'static MultibootInfo) -> ! {
    // Check that multiboot magic number is correct.
    assert_eq!(magic, MULTIBOOT_BOOTLOADER_MAGIC);

    // Initialize the kernel.
    eciton::init_kernel(boot_info);

    // Halt kernel.
    loop {}
}