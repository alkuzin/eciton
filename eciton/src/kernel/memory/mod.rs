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

//! Kernel memory management main module.

mod manager;
mod layout;

use super::{bitmap::Bitmap, bitops::bits_to_bytes};
use crate::{pr_debug, BOOT_INFO};
use manager::MemoryManager;
use layout::*;

/// Page size in bytes.
pub const PAGE_SIZE: usize = 4096;

/// Page bit shift.
pub const PAGE_SHIFT: u8 = 0xC;

/// Convert physical address to page frame number.
///
/// # Parameters
/// - `addr` - given page physical address.
///
/// # Returns
/// Page frame number.
#[inline(always)]
fn phys_to_page_num(addr: u32) -> usize {
    addr as usize >> PAGE_SHIFT
}

/// Initialize physical memory manager.
pub fn init() {
    let boot_info = BOOT_INFO.lock();

    // Check that multiboot memory map is set correctly.
    if (boot_info.flags & (1 << 6)) == 0 {
        panic!("Multiboot memory map wasn't set correctly!");
    }

    let mut mm = manager::MM.lock();
    mm.detect_memory(&boot_info);

    // In order to prevent overwriting stack with bitmap data,
    // bitmap will start right after the kernel stack.
    let kernel_end_ptr = kernel_end_vaddr() as *const u32;
    let bitmap_addr    = unsafe { kernel_end_ptr.add(STACK_SIZE) as *mut u32};
    let bitmap_size    = bits_to_bytes(mm.max_pages);
    let bitmap_data    = mm.bitmap.data;

    // Physical memory bitmap starts right after the kernel end.
    mm.bitmap = Bitmap::new(bitmap_addr, bitmap_size);

    unsafe {
        // Mark all memory as used.
        bitmap_addr.write_bytes(0xFF, bitmap_size);
        mm.used_pages = mm.max_pages;
    }

    mm.free_available_memory(&boot_info);

    // Mark kernel memory as used.
    mm.mark_as_used(kernel_begin_paddr(), kernel_size as usize + PAGE_SIZE);

    // Mark memory between kernel end & bitmap as used.
    mm.mark_as_used(bitmap_addr.wrapping_sub(STACK_SIZE) as u32, STACK_SIZE);

    // Mark bitmap memory as used.
    mm.mark_as_used(bitmap_data as u32, bitmap_size);

    // These pages containing reserved data that should not
    // be accessed, so it was set as used:
    mm.reserve_page(0);    // Containing GDT.
    mm.reserve_page(16);   // Containing multiboot info structure.

    // Output for debug purpose.
    print_memory_info(&mm, bitmap_addr, bitmap_size);
}

/// Print kernel memory info.
///
/// # Parameters
/// - `mm`      - given memory manager struct.
/// - `bm_addr` - given bitmap address.
/// - `bm_size` - given bitmap size in bytes.
#[doc(hidden)]
fn print_memory_info(mm: &MemoryManager, bm_addr: *const u32, bm_size: usize) {
    pr_debug!("Kernel layout:");
    pr_debug!("[mem {:#010X}-{:#010X}] Physical memory.",
        kernel_begin_paddr(),kernel_end_paddr()
    );
    pr_debug!("[mem {:#010X}-{:#010X}] Virtual memory.",
        kernel_begin_vaddr(), kernel_end_vaddr()
    );
    pr_debug!("Base address: <{:#010X}>.", base_vaddr());
    pr_debug!("Kernel size: {} bytes.", kernel_size());
    pr_debug!("Stack size:  {} bytes.", STACK_SIZE);
    pr_debug!("Total RAM:   {} KB.", mm.mem_total >> 0xA);
    pr_debug!("Max pages:   {}.", mm.max_pages);
    pr_debug!("Set bitmap at address: <{:010p}>.", bm_addr);
    pr_debug!("Set bitmap size: {} bytes.", bm_size);
}