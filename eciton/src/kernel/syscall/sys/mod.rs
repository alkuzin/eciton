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

//! Kernel syscall implementations main module.

pub mod allocpg;
pub mod getfb;
pub mod null;
pub mod freepg;

use eciton_sdk::{arch::i686::{irq::IntRegisterState}};
pub use allocpg::allocpg;
pub use freepg::freepg;
pub use getfb::getfb;
pub use null::null;

/// Syscall result enumeration.
#[derive(Debug)]
#[repr(u32)]
pub enum SyscallResult {
    /// This field is used if syscall operation is successful.
    Success = 0,
    /// This field is used if syscall operation failed.
    Error = u32::MAX,
}

use crate::test::*;

exotest! {
    exotest_test_cases! {
        test_syscalls, {
            exotest_run_modules!(
                null,
                getfb,
                allocpg,
                freepg
            );
        }
    }
}
