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

//! Kernel syscall handler module.

mod sys;

use crate::{
    kernel::arch::i686::irq::{self, IntRegisterState},
    pr_debug, pr_err
};
use eciton_sdk::arch::i686::irq::InterruptHandler;

/// Eciton exokernel syscall interrupt number.
pub const SYSCALL_NUM: usize = 0x66;

/// Total number of system calls.
const SYSCALL_COUNT: usize = 4;

/// Table of syscall functions.
static SYSCALL_TABLE: [InterruptHandler;SYSCALL_COUNT] = [
    sys::null,
    sys::getfb,
    sys::allocpg,
    sys::freepg,
];

/// Handle syscalls.
///
/// # Parameters
/// - `regs` - given pointer to interrupt register state.
fn syscall_handler(regs: &mut IntRegisterState) {
    pr_debug!("SYSCALL HANDLER BEGIN");

    let syscall_number = regs.eax as usize;
    let syscall_option = SYSCALL_TABLE.get(syscall_number);

    match syscall_option {
        Some(syscall) => syscall(regs),
        None => {
            pr_err!("Incorrect syscall number: {}", syscall_number);
        }
    }

    pr_debug!("SYSCALL HANDLER END");
}

/// Initialize syscall handler.
pub fn init() {
    irq::request(SYSCALL_NUM, syscall_handler);
}

// TODO: put contents of exotest to tests module.

use crate::tests::*;

exotest! {
    exotest_test_cases! {
        test_syscalls, {
            sys::null::run_tests();
            sys::getfb::run_tests();
            sys::allocpg::run_tests();
            sys::freepg::run_tests();
        }
    }
}