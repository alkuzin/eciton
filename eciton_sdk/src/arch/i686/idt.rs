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

//! Provides definitions for Interrupt Descriptor Table (IDT).
//!
//! # Description
//! The Interrupt Descriptor Table (IDT) is a crucial data structure in x86
//! architecture that is used to manage interrupts and exceptions.
//! It allows the CPU to respond to various events, such as hardware
//! interrupts, software interrupts, and exceptions.

/// IDT gate descriptor structure in 32-bit mode.
#[derive(Debug, Default)]
#[repr(C, packed)]
pub struct Entry {
    /// Entry point of the ISR (lower bits).
    pub offset_low: u16,
    /// Point to a valid code segment in GDT.
    pub selector: u16,
    /// Unused.
    pub reserved: u8,
    /// Gate type & other control bits.
    pub flags: u8,
    /// Entry point of the ISR (higher bits).
    pub offset_high: u16,
}

impl Entry {
    /// Construct new IDT entry.
    ///
    /// # Parameters
    /// - `offset`   - given entry point of the ISR.
    /// - `selector` - given point to a valid code segment in GDT.
    /// - `flags`    - given gate type & other control bits.
    pub fn new(offset: u32, selector: u16, flags: u8) -> Self {
        let mut entry     = Entry::default();
        entry.offset_low  = (offset & 0xFFFF) as u16;
        entry.selector    = selector;
        entry.reserved    = 0;
        entry.flags       = flags | 0x60;
        entry.offset_high = ((offset >> 0x10) & 0xFFFF) as u16;
        entry
    }
}

/// IDT pointer.
#[derive(Debug, Default)]
#[repr(C, packed)]
pub struct Pointer {
    /// Size of IDT.
    pub size: u16,
    /// The linear address of the IDT.
    pub offset: u32,
}