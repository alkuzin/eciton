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

#![no_main]
#![no_std]

pub mod multiboot;
pub mod init;

use multiboot::MultibootInfo;

static GREETING: &[u8] = b"Eciton exokernel v0.0.0";

#[unsafe(no_mangle)]
extern "C" fn kmain(_magic: u32, _mboot: &MultibootInfo) -> ! {

    assert_eq!(_magic, multiboot::MULTIBOOT_BOOTLOADER_MAGIC);

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in GREETING.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xE;
        }
    }

    loop {}
}