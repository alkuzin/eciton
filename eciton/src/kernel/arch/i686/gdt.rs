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

use crate::pr_debug;
use core::fmt;

/// GDT segment structure in 32-bit mode.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
struct Entry {
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

/// GDT pointer.
#[repr(C, packed)]
struct Pointer {
    /// GDT size - 1.
    size: u16,
    /// Linear address of GDT.
    offset: u32,
}

/// GDT segment offsets enumeration.
#[repr(u8)]
#[derive(Debug)]
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
#[repr(u8)]
enum Access {
    KernelCode  = 0x9A,
    KernelData  = 0x92,
    KernelStack = 0x97,
    UserCode    = 0xFA,
    UserData    = 0xF2,
    UserStack   = 0xF7,
}

/// GDT pointer physical address.
const GDT_BASE: u32 = 0x800;

/// Number of GDT entries.
const GDT_ENTRIES: usize = 7;

/// Empty entry.
const NULL_ENTRY: Entry = Entry {
    limit:      0,
    base_low:   0,
    base_mid:   0,
    access:     0,
    flags:      0,
    base_high:  0,
};

/// Global Descriptor Table.
static mut GDT: [Entry;GDT_ENTRIES] = [NULL_ENTRY;GDT_ENTRIES];

/// Global Descriptor Table pointer.
static mut GDT_PTR: *mut Pointer = GDT_BASE as *mut Pointer;

/// Set the GDT entry.
///
/// # Parameters
/// - `eno`    - given GDT entry number.
/// - `base`   - given linear address where the segment begins.
/// - `limit`  - given maximum addressable unit.
/// - `access` - given segment access byte.
/// - `flags`  - given segment flags.
fn set_entry(eno: usize, base: u32, limit: u32, access: u8, flags: u8) {
    unsafe {
        GDT[eno].base_low  = (base & 0xFFFF) as u16;
        GDT[eno].base_mid  = ((base >> 0x10) & 0xFF) as u8;
        GDT[eno].base_high = ((base >> 0x18) & 0xFF) as u8;
        GDT[eno].limit     = (limit & 0xFFFF) as u16;
        GDT[eno].flags     = ((limit >> 0x10) & 0x0F) as u8;
        GDT[eno].flags     |= flags & 0xF0;
        GDT[eno].access    = access;
    }
}

unsafe extern "C" {
    /// Flush out the old GDT and install the new changes.
    ///
    /// # Parameters
    /// - `ptr` - given new GDT pointer to update.
    unsafe fn gdt_flush(ptr: u32);
}

/// Initialize Global Descriptor Table.
pub fn init() {
    // 32-bit protected mode segment.
    const FLAGS: u8  = 0xCF;
    const BASE: u32  = 0x00000000;
    const LIMIT: u32 = 0xFFFFFFFF;

    // Set GDT entries.
    // (Null descriptor) should always contain no data.
    set_entry(0, 0, 0, 0, 0);

    // Kernel space segments.
    set_entry(1, BASE, LIMIT, Access::KernelCode  as u8, FLAGS);
    set_entry(2, BASE, LIMIT, Access::KernelData  as u8, FLAGS);
    set_entry(3, BASE, LIMIT, Access::KernelStack as u8, FLAGS);

    // User space segments.
    set_entry(4, BASE, LIMIT, Access::UserCode  as u8, FLAGS);
    set_entry(5, BASE, LIMIT, Access::UserData  as u8, FLAGS);
    set_entry(6, BASE, LIMIT, Access::UserStack as u8, FLAGS);

    unsafe {
        // Set GDT pointer.
        let gdt_ptr    = GDT_PTR.as_mut().unwrap();
        gdt_ptr.size   = (size_of::<Entry>() * GDT_ENTRIES - 1) as u16;
        gdt_ptr.offset = (&raw const GDT as *const _) as u32;

        // Update GDT.
        gdt_flush(GDT_BASE);
    }

    print_gdt();
}

/// Print GDT related info for debug.
#[doc(hidden)]
fn print_gdt() {
    pr_debug!("GDT pointer: <{:#?}> {:#?}",
        unsafe { GDT_PTR },
        unsafe { GDT_PTR.as_ref().unwrap() }
    );

    pr_debug!("Set {} GDT entries", GDT_ENTRIES);

    for (i, entry) in unsafe { GDT }.iter().enumerate() {
        pr_debug!("GDT entry {}:  Access: {:#08X}  Flags: {:#08X}",
            i,
            entry.access,
            entry.flags,
        );
    }
}

/// Custom debug output for GDT pointer.
impl fmt::Debug for Pointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size   = self.size;
        let offset = self.offset;
        write!(f, "[size: {} bytes, offset: <{:#08x}>]", size, offset)?;

        Ok(())
    }
}
