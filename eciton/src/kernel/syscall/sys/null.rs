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

//! Null syscall implementation.

use super::{IntRegisterState, SyscallResult};

/// Null syscall to do nothing.
///
/// # Parameters
/// - `regs` - given pointer to interrupt register state.
pub fn null(regs: &mut IntRegisterState) {
    // Put success return value into eax register.
    regs.eax = SyscallResult::Success as u32;
}

use crate::tests::*;

exotest! {
    exotest_test_cases! {
        test_null_syscall, {
            let mut regs = IntRegisterState::default();
            null(&mut regs);

            // Check return value.
            let ret = regs.eax;
            assert_eq!(ret, SyscallResult::Success as u32);
        }
    }
}
