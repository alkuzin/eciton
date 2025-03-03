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

//! Eciton exokernel syscall API main module.

use crate::sdk::Framebuffer;
use core::arch::asm;

// TODO: make syscalls compatible with C.

/// Get framebuffer syscall number.
const SYSCALL_GETFB: u32 = 1;

/// Get framebuffer info.
///
/// # Parameters
/// - `fb` - given framebuffer info struct to fill.
///
/// # Returns
/// - `0`  - in case of success.
/// - `-1` - otherwise.
pub fn getfb(fb: &mut Framebuffer) -> i32 {
    let ret: i32;
    unsafe {
        asm!("int 0x66", in("eax") SYSCALL_GETFB, in("ebx") fb);

        // Get return value from eax register.
        asm!("mov {0:e}, eax", out(reg) ret);
    }
    ret
}