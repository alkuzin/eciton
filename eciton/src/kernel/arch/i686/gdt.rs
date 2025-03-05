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

use eciton_sdk::arch::i686::gdt::{Entry, Pointer, Access};
use crate::pr_debug;

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

/// Initialize Global Descriptor Table.
pub fn init() {
    set_entries();
    set_pointer();

    unsafe {
        // Update GDT.
        gdt_flush(GDT_BASE);
    }

    print_gdt();
}

/// Set GDT entries.
fn set_entries() {
    // 32-bit protected mode segment.
    const FLAGS: u8  = 0xCF;
    const BASE: u32  = 0x00000000;
    const LIMIT: u32 = 0xFFFFFFFF;

    let null       = Entry::default();
    let kern_code  = Entry::new(BASE, LIMIT, Access::KernelCode  as u8, FLAGS);
    let kern_data  = Entry::new(BASE, LIMIT, Access::KernelData  as u8, FLAGS);
    let kern_stack = Entry::new(BASE, LIMIT, Access::KernelStack as u8, FLAGS);
    let user_code  = Entry::new(BASE, LIMIT, Access::UserCode    as u8, FLAGS);
    let user_data  = Entry::new(BASE, LIMIT, Access::UserData    as u8, FLAGS);
    let user_stack = Entry::new(BASE, LIMIT, Access::UserStack   as u8, FLAGS);

    unsafe {
        // (Null descriptor) should always contain no data.
        GDT[0] = null;

        // Kernel space segments.
        GDT[1] = kern_code;
        GDT[2] = kern_data;
        GDT[3] = kern_stack;

        // User space segments.
        GDT[4] = user_code;
        GDT[5] = user_data;
        GDT[6] = user_stack;
    }
}

// Set GDT pointer.
fn set_pointer() {
    unsafe {
        let gdt_ptr    = GDT_PTR.as_mut().unwrap();
        gdt_ptr.size   = (size_of::<Entry>() * GDT_ENTRIES - 1) as u16;
        gdt_ptr.offset = (&raw const GDT as *const _) as u32;
    }
}

unsafe extern "C" {
    /// Flush out the old GDT and install the new changes.
    ///
    /// # Parameters
    /// - `ptr` - given new GDT pointer to update.
    unsafe fn gdt_flush(ptr: u32);
}

/// Print GDT related info for debug.
#[doc(hidden)]
fn print_gdt() {
    let gdt_ptr = unsafe { GDT_PTR.as_ref().unwrap() };
    let size    = gdt_ptr.size;
    let offset  = gdt_ptr.offset;

    pr_debug!("GDT pointer: <{:#?}> [size: {} bytes, offset: <{:#08x}>]",
        unsafe { GDT_PTR }, size, offset
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