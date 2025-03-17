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

use crate::{kernel::memory::free_pages, pr_err};
use super::{IntRegisterState, SyscallResult};

/// Free memory pages.
///
/// # Parameters
/// - `regs` - given pointer to interrupt register state.
pub fn freepg(regs: &mut IntRegisterState) {
    // Get order (allocating 2^order pages).
    let addr  = regs.ebx;
    let order = regs.ecx;

    if addr == 0 || order == 0 {
        // Error return value -1.
        regs.eax = SyscallResult::Error as u32;
        return;
    }

    // TODO: check range of order.
    match free_pages(addr, order) {
        Ok(_)  => {
            // Put return value into eax register.
            regs.eax = SyscallResult::Success as u32;
        },
        Err(_) => {
            pr_err!("Error to free 2^{order} pages at address <{addr}>");
            // Error return value -1.
            regs.eax = SyscallResult::Error as u32;
        }
    }
}
