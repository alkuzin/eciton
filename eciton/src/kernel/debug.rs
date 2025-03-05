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
    kernel::arch::i686::register::{self, RegisterState}, pr_panic, printk, putk
};
use core::{
    slice::from_raw_parts,
    str::from_utf8,
};

/// Print current CPU registers state.
pub fn dump_registers() {
    let state = RegisterState::new();

    pr_panic!("EAX: {:#010X}  EBX: {:#010X}  ECX: {:#010X}  EDX: {:#010X}",
        state.eax,
        state.ebx,
        state.ecx,
        state.edx,
    );

    pr_panic!("ESI: {:#010X}  EDI: {:#010X}  EBP: {:#010X}  ESP: {:#010X}",
        state.esi,
        state.edi,
        state.ebp,
        state.esp,
    );

    pr_panic!("DS:  {:#010X}  ES:  {:#010X}  FS:  {:#010X}",
        state.ds,
        state.es,
        state.fs,
    );

    pr_panic!("GS:  {:#010X}  CS:  {:#010X}  SS:  {:#010X}",
        state.gs,
        state.cs,
        state.ss,
    );

    pr_panic!("CR0: {:#010X}  CR2: {:#010X}  CR3: {:#010X}",
        state.cr0,
        state.cr2,
        state.cr3,
    );

    pr_panic!("EIP: {:#010X}", state.eip);
    pr_panic!("EFLAGS: {:#010X}", state.eflags);

    for flag in register::EFLAGS.iter() {
        if (state.eflags & *flag as u32) != 0 {
            pr_panic!("- {:?}", &flag);
        }
    }
}

/// Convert byte to ASCII.
///
/// # Parameters
/// - `ch` - given byte to convert.
///
/// # Returns
/// ASCII character to print.
#[inline(always)]
fn to_print(ch: u8) -> char {
    if ch.is_ascii_graphic() {ch as char} else {'.'}
}

/// Dump file.
///
/// # Parameters
/// - `addr` - given physical address.
/// - `size` - given number of bytes to dump.
pub fn kdump(addr: u32, size: usize) {
    const BYTES_PER_LINE: usize       = 16;
    const BYTES_PER_LINE_SHIFT: usize = 0x4;

    let ptr: *const u8  = addr as *const u8;
    let buffer          = unsafe { from_raw_parts(ptr, size) };
    let total_lines     = (size >> BYTES_PER_LINE_SHIFT) + 1;
    let mut rows: usize = 0;

    let mut line: [u8;BYTES_PER_LINE] = [0;BYTES_PER_LINE];

    // Print current total number of bytes printed.
    putk!("{:08x} ", rows);

    // Print physical address of current line.
    putk!("<{:#010p}>  ", unsafe { ptr.add(rows) });

    for (i, byte) in buffer.iter().enumerate() {
        putk!("{:02x} ", byte);
        line[i % 16] = to_print(*byte) as u8;

        // Print extra space between each 8 bytes.
        if (i + 1) % 8 == 0 {
            putk!(" ");
        }

        // Print next line each 16 bytes.
        if (i + 1) % BYTES_PER_LINE == 0 {
            rows += BYTES_PER_LINE;

            // Print string representation of current line.
            printk!("|{}|", from_utf8(&line).unwrap());
            line.fill(0);

            // Check that current line is not the last.
            if rows >> BYTES_PER_LINE_SHIFT < total_lines - 1 {
                // Print current total number of bytes printed.
                putk!("{:08x} ", rows);

                // Print physical address of current line.
                putk!("<{:#010p}>  ", unsafe { ptr.add(rows) });
            }
            else {
                // Go to the next line if current line is the last.
                printk!();
            }
        }
    }
}