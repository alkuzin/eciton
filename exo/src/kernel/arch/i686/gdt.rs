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

//! Global Decriptor Table declaration module.

// use crate::eciton::printk;

/// GDT pointer physical address.
const GDT_BASE: u32 = 0x800;

/// GDT segment offsets enumeration.
#[repr(u8)]
enum Segment {
    Null        = 0x00,
    KernelCode  = 0x08,
    KernelData  = 0x10,
    KernelStack = 0x18,
    UserCode    = 0x20,
    UserData    = 0x28,
    UserStack   = 0x30,
}

/// GDT segment entry structure.
#[derive(Debug, Default)]
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
    /// Segment mode
    pub flags: u8,
    /// Linear address where the segment begins.
    pub base_high: u8,
}

impl Entry {
    pub fn new(base: u32, limit: u32, access: u8, flags: u8) -> Entry {
        let base_low  = (base & 0xFFFF) as u16;
        let base_mid  = ((base >> 0x10) & 0xFF) as u8;
        let base_high = ((base >> 0x18) & 0xFF) as u8;
        let limit     = (limit & 0xFFFF) as u16;

        // Using u32 for the shift operation in order to
        // prevent arithmetic overflow:
        let tmp   = ((limit as u32) >> 0x10) & 0x0F;
        let flags = tmp as u8 | (flags & 0xF0);

        Entry { base_low, base_mid, base_high, limit, flags, access }
    }
}

/// GDT pointer.
#[repr(C, packed)]
struct Pointer {
    /// GDT size - 1.
    pub size: u16,
    /// linear address of GDT.
    pub offset: u32,
}

/// Access bytes enumeration.
#[repr(u8)]
enum Access {
    KernelCode  = 0x9A,
    KernelData  = 0x92,
    KernelStack = 0x97,
    UserCode    = 0xFA,
    UserData    = 0xF2,
    UserStack   = 0xF7,
}

const ENTRIES_COUNT: usize = 7;

// WARNING: that should be implemented in libOS:
pub fn init() {
    // 32-bit protected mode segment.
    const FLAGS:u8  = 0xCF;
    const BASE:u32  = 0x00000000;
    const LIMIT:u32 = 0xFFFFFFFF;

    // Set GDT entries:
    let _gdt: [Entry;ENTRIES_COUNT] = [
        // (Null descriptor) should always contain no data
        Entry::new(0, 0, 0, 0),

        // Kernel space segments:
        Entry::new(BASE, LIMIT, Access::KernelCode  as u8, FLAGS),
        Entry::new(BASE, LIMIT, Access::KernelData  as u8, FLAGS),
        Entry::new(BASE, LIMIT, Access::KernelStack as u8, FLAGS),

        // User space segments:
        Entry::new(BASE, LIMIT, Access::UserCode    as u8, FLAGS),
        Entry::new(BASE, LIMIT, Access::UserData    as u8, FLAGS),
        Entry::new(BASE, LIMIT, Access::UserStack   as u8, FLAGS),
    ];
}