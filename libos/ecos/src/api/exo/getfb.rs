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

//! Get framebuffer syscall implementation.

use eciton_sdk::vbe::Framebuffer;
use super::{syscall, SyscallArgs, Syscall};

/// Get framebuffer info.
///
/// # Parameters
/// - `fb` - given framebuffer info struct to fill.
///
/// # Returns
/// - `Ok`  - in case of success.
/// - `Err` - otherwise.
pub fn getfb() -> Result<Framebuffer, ()> {
    // Set syscall arguments.
    let mut args = SyscallArgs::default();
    args.arg1    = Syscall::Getfb as u32;

    let fb    = Framebuffer::default();
    args.arg2 = (&fb as *const _) as u32;

    // Get syscall output.
    let output = syscall(&args);
    let ret    = output.arg1;

    // Handle return value.
    match ret {
        0 => Ok(fb),
        _ => Err(()),
    }
}
