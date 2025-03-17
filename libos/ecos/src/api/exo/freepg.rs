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

//! Free memory pages syscall implementation.

use super::{syscall, SyscallArgs, Syscall, allocpg::AllocUnit};

/// Free memory pages.
///
/// # Parameters
/// - `unit` - given allocation unit (allocated by allopg syscall).
///
/// # Returns
/// - `Ok`  - in case of success.
/// - `Err` - otherwise.
pub fn freepg(unit: AllocUnit) -> Result<(), ()> {
    // Set syscall arguments.
    let mut args = SyscallArgs::default();
    args.arg1    = Syscall::FreePg as u32;
    args.arg2    = unit.addr;
    args.arg3    = unit.order;

    // Get syscall output.
    let output = syscall(&args);
    let ret    = output.arg1;

    // Handle return value.
    match ret {
        0 => Ok(()),
        _ => Err(()),
    }
}
