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

use super::{IntRegisterState, SyscallResult};
use crate::kernel::memory::alloc_pages;

/// Allocate memory pages.
///
/// # Parameters
/// - `regs` - given pointer to interrupt register state.
pub fn allocpg(regs: &mut IntRegisterState) {
    // Get number of pages to allocate.
    let count  = regs.ebx;
    let result = alloc_pages(count);

    match result {
        Ok(addr) => {
            // Put return value into eax register.
            regs.eax = SyscallResult::Success as u32;

            // Put memory page address.
            regs.ebx = addr;
        }
        Err(_) => {
            // Error return value -1.
            regs.eax = SyscallResult::Error as u32;
        }
    }
}

use crate::tests::*;

exotest! {
    use crate::kernel::memory::free_pages;

    exotest_test_cases! {
        test_allocpg_syscall_successful, {
            let mut regs = IntRegisterState::default();
            let count = 6;
            regs.ebx  = count;
            allocpg(&mut regs);

            // Check return value.
            let ret = regs.eax;
            assert_eq!(ret, SyscallResult::Success as u32);

            // Check memory address.
            let addr = regs.ebx;
            assert_ne!(addr, 0);

            let _ = free_pages(addr, count);
        },

        test_allocpg_syscall_allocate_zero_pages, {
            let mut regs = IntRegisterState::default();
            let count = 0;
            regs.ebx  = count;
            allocpg(&mut regs);

            // Check return value.
            let ret = regs.eax;
            assert_eq!(ret, SyscallResult::Error as u32);
        },

        test_allocpg_syscall_allocate_too_many_pages, {
            let mut regs = IntRegisterState::default();
            let count = 1337;
            regs.ebx  = count;
            allocpg(&mut regs);

            // Check return value.
            let ret = regs.eax;
            assert_eq!(ret, SyscallResult::Error as u32);
        }
    }
}
