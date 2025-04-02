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

//! Allocate memory pages syscall implementation.

use super::{syscall, SyscallArgs, Syscall};

#[derive(Debug, Default, Copy, Clone)]
pub struct AllocUnit {
    /// Memory page address.
    pub addr: u32,
    /// Number of pages.
    pub count: u32,
}

/// Allocate memory pages.
///
/// # Parameters
/// - `count` - given number of pages to allocate.
///
/// # Returns
/// - `AllocUnit` - in case of success.
/// - `Err`       - otherwise.
pub fn allocpg(count: u32) -> Result<AllocUnit, ()> {
    // Set syscall arguments.
    let mut args = SyscallArgs::default();
    args.arg1    = Syscall::AllocPg as u32;
    args.arg2    = count;

    // Get syscall output.
    let output = syscall(&args);
    let ret    = output.arg1;
    let addr   = output.arg2;

    // Handle return value.
    match ret {
        0 => Ok(AllocUnit {addr, count}),
        _ => Err(()),
    }
}