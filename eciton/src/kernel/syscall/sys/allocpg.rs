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

//! Alloc memory pages syscall implementation.

use crate::{kernel::memory::alloc_pages, pr_err};
use super::{IntRegisterState, SyscallResult};

/// Allocate memory pages.
///
/// # Parameters
/// - `regs` - given pointer to interrupt register state.
pub fn allocpg(regs: &mut IntRegisterState) {
    // Get order (allocating 2^order pages).
    let order = regs.ebx;

    // TODO: check range of order.
    let addr = alloc_pages(order).unwrap_or_else(|_| {
        pr_err!("Error to allocate 2^{order} pages");
        0
    });

    if addr == 0 {
        // Error return value -1.
        regs.eax = SyscallResult::Error as u32;
        return;
    }

    // Put return value into eax register.
    regs.eax = SyscallResult::Success as u32;

    // Put memory page address.
    regs.ebx = addr;
}
