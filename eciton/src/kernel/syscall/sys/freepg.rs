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
    let addr  = regs.ebx;
    let count = regs.ecx;

    if addr == 0 || count == 0 {
        // Error return value -1.
        regs.eax = SyscallResult::Error as u32;
        return;
    }

    match free_pages(addr, count) {
        Ok(_)  => {
            // Put return value into eax register.
            regs.eax = SyscallResult::Success as u32;
        },
        Err(_) => {
            pr_err!("Error to free {count} pages at address <{addr}>");
            // Error return value -1.
            regs.eax = SyscallResult::Error as u32;
        }
    }
}

use crate::test::*;

exotest! {
    use crate::kernel::memory::{alloc_pages, PAGE_LIMIT};

    exotest_test_cases! {
        test_freepg_syscall_successful, {
            let count = 6;
            let addr  = alloc_pages(count).unwrap();

            let mut regs = IntRegisterState::default();
            regs.ebx     = addr;
            regs.ecx     = count;
            freepg(&mut regs);

            // Check return value.
            let ret = regs.eax;
            assert_eq!(ret, SyscallResult::Success as u32);

            let _ = free_pages(addr, count);
        },

        test_freepg_syscall_free_zero_pages, {
            let count = 0;

            let mut regs = IntRegisterState::default();
            regs.ebx     = 0;
            regs.ecx     = count;
            freepg(&mut regs);

            // Check return value.
            let ret = regs.eax;
            assert_eq!(ret, SyscallResult::Error as u32);
        },

        test_freepg_syscall_free_incorrect_address, {
            let count = 6;

            let mut regs = IntRegisterState::default();
            regs.ebx     = 0;
            regs.ecx     = count;
            freepg(&mut regs);

            // Check return value.
            let ret = regs.eax;
            assert_eq!(ret, SyscallResult::Error as u32);
        },

        test_freepg_syscall_free_too_many_pages, {
            let count = PAGE_LIMIT as u32;

            let mut regs = IntRegisterState::default();
            regs.ebx     = 0;
            regs.ecx     = count;
            freepg(&mut regs);

            // Check return value.
            let ret = regs.eax;
            assert_eq!(ret, SyscallResult::Error as u32);
        }
    }
}
