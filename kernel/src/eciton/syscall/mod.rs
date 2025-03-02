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

//! Kernel syscalls module.

use crate::{
    eciton::arch::i686::irq::{self, IntRegisterState},
    pr_debug, pr_err
};

pub const SYSCALL_NUM: usize = 0x66;
const SYSCALL_COUNT: usize   = 1;

// TODO: add syscall table

pub fn init() {
    irq::request(SYSCALL_NUM, syscall_handler);
}

fn syscall_handler(regs: &IntRegisterState) {
    pr_debug!("SYSCALL HANDLER BEGIN");
    pr_debug!("{:#X?}", regs);

    match regs.eax {
        _ => pr_err!("Incorrect syscall number"),
    }

    pr_debug!("SYSCALL HANDLER END");
}