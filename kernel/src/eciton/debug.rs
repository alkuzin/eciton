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

//! Kernel debug related functions module.

use crate::{
    eciton::arch::i686::register::{self, RegisterState},
    printk,
    putk
};

/// Print current CPU registers state.
pub fn dump_registers() {
    let state = RegisterState::new();

    printk!("EAX: {:#010X}  EBX: {:#010X}  ECX: {:#010X}  EDX: {:#010X}",
        state.eax,
        state.ebx,
        state.ecx,
        state.edx,
    );

    printk!("ESI: {:#010X}  EDI: {:#010X}  EBP: {:#010X}  ESP: {:#010X}",
        state.esi,
        state.edi,
        state.ebp,
        state.esp,
    );

    printk!("DS:  {:#010X}  ES:  {:#010X}  FS:  {:#010X}",
        state.ds,
        state.es,
        state.fs,
    );

    printk!("GS:  {:#010X}  CS:  {:#010X}  SS:  {:#010X}",
        state.gs,
        state.cs,
        state.ss,
    );

    printk!("CR0: {:#010X}  CR2: {:#010X}  CR3: {:#010X}",
        state.cr0,
        state.cr2,
        state.cr3,
    );

    putk!("EIP: {:#010X}  EFLAGS: {:#010X} [ ", state.eip, state.eflags);

    for i in register::EFLAGS.iter() {
        if (state.eflags & i.mask) != 0 {
            putk!("{} ", i.label);
        }
    }

    printk!("]");
}