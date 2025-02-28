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

//! Contains functions for input/output operations on ports.

use core::arch::asm;

/// Receive a byte of data from a specified input/output port.
///
/// # Parameters
/// - `port` - given port from which the data will be read.
///
/// # Returns
///  The byte of data read from the port.
#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let mut ret: u8;
    unsafe {
        asm!("in al, dx", out("al") ret, in("dx") port);
    }
    ret
}

/// Output a byte to a specified port.
///
/// # Parameters
/// - `port` - given port to which the data will be written.
/// - `data` - given data byte to be written to the port.
#[inline]
pub unsafe fn outb(port: u16, data: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") data);
    }
}