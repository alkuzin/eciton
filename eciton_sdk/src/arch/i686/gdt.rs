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

//! Global Decriptor Table module.
//!
//! # Description
//! The Global Descriptor Table (GDT) is a critical data structure used in x86
//! architecture for memory management and protection. It defines the
//! characteristics of various memory segments, allowing the CPU to manage
//! memory access and enforce protection mechanisms.

/// GDT segment structure in 32-bit mode.
#[derive(Debug, Default, Clone, Copy)]
#[repr(C, packed)]
pub struct Entry {
    /// Maximum addressable unit.
    pub limit: u16,
    /// Linear address where the segment begins.
    pub base_low: u16,
    /// Linear address where the segment begins.
    pub base_mid: u8,
    /// Privileges of segment.
    pub access: u8,
    /// Segment mode.
    pub flags: u8,
    /// Linear address where the segment begins.
    pub base_high: u8,
}

impl Entry {
    /// Construct new GDT entry.
    ///
    /// # Parameters
    /// - `base`   - given linear address where the segment begins.
    /// - `limit`  - given maximum addressable unit.
    /// - `access` - given segment access byte.
    /// - `flags`  - given segment flags.
    pub fn new(base: u32, limit: u32, access: u8, flags: u8) -> Self {
        let mut entry   = Entry::default();
        entry.base_low  = (base & 0xFFFF) as u16;
        entry.base_mid  = ((base >> 0x10) & 0xFF) as u8;
        entry.base_high = ((base >> 0x18) & 0xFF) as u8;
        entry.limit     = (limit & 0xFFFF) as u16;
        entry.flags     = ((limit >> 0x10) & 0x0F) as u8;
        entry.flags     |= flags & 0xF0;
        entry.access    = access;
        entry
    }
}

/// GDT pointer.
#[derive(Debug)]
#[repr(C, packed)]
pub struct Pointer {
    /// GDT size - 1.
    pub size: u16,
    /// Linear address of GDT.
    pub offset: u32,
}

/// GDT segment offsets enumeration.
#[derive(Debug)]
#[repr(u8)]
pub enum Segment {
    Null        = 0x00,
    KernelCode  = 0x08,
    KernelData  = 0x10,
    KernelStack = 0x18,
    UserCode    = 0x20,
    UserData    = 0x28,
    UserStack   = 0x30,
}

/// Access bytes enumeration.
#[derive(Debug)]
#[repr(u8)]
pub enum Access {
    KernelCode  = 0x9A,
    KernelData  = 0x92,
    KernelStack = 0x97,
    UserCode    = 0xFA,
    UserData    = 0xF2,
    UserStack   = 0xF7,
}