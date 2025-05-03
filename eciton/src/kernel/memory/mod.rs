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

use eciton_sdk::{collections::Bitmap, bitops::bits_to_bytes};
use crate::{pr_debug, pr_err, BOOT_INFO};
use core::{ffi::c_void, ptr};
use manager::MemoryManager;
use layout::*;

// TODO: move Page struct to EcitonSDK
// TODO: replace with Page struct

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

/// Convert page frame number to physical address.
///
/// # Parameters
/// - `addr` - given page frame number.
///
/// # Returns
/// Page frame physical address.
#[inline(always)]
fn page_num_to_phys(pfn: usize) -> u32 {
    (pfn << PAGE_SHIFT) as u32
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

    // Physical memory bitmap starts right after the kernel end.
    mm.bitmap = Bitmap::from_pointer(bitmap_addr, bitmap_size);
    let bitmap_data = mm.bitmap.as_pointer().unwrap();

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

/// Constant representing free object in bitmap.
const PAGE_FREE: bool = false;

/// Constant representing used object in bitmap.
const PAGE_USED: bool = true;

/// Get free pages.
///
/// # Parameters
/// - `count` - given number of pages to find.
///
/// # Returns
/// Page position in bitmap - in case of success.
/// Err() - otherwise.
fn get_free_pages(mm: &MemoryManager, count: u32) -> Result<usize, ()> {
    // Number of free pages to find.
    let n = count as usize;
    let bits_per_element = mm.bitmap.bits_per_element();

    #[allow(unused_assignments)]
    let mut pos: usize = 0;

    for i in 0..mm.bitmap.capacity() {
        // Skip groups of used pages.
        let group = unsafe { *mm.bitmap.as_pointer().unwrap().add(i) };

        if group != 0xFFFFFFFF {

            // Handle each group.
            for j in 0..bits_per_element {
                pos = bits_per_element * i + j;

                // Skip until free page.
                while mm.bitmap.get(pos) == PAGE_USED {
                    pos += 1;
                }

                if mm.bitmap.get(pos) == PAGE_FREE {
                    // Check that number of free pages equals to
                    // the number of needed pages (n).
                    let mut is_found = false;

                    for k in 0..n {
                        if mm.bitmap.get(pos + k) == PAGE_USED {
                            // Used page is found.
                            is_found = true;
                            break;
                        }
                    }

                    // If used page was found check next group of pages.
                    if is_found {
                        continue;
                    }
                    else {
                        return Ok(pos);
                    }
                }
            }
        }
    }

    Err(())
}

/// Allocate free zeroed pages.
///
/// # Parameters
/// - `count` - given number of pages to allocate.
///
/// # Returns
/// Page physical address - in case of success.
/// Err - otherwise.
pub fn alloc_pages(count: u32) -> Result<u32, ()> {
    let mut mm = manager::MM.lock();
    let n      = count as usize;

    // Handle incorrect number of pages.
    if n >= mm.max_pages {
        pr_err!("Page count exceed total number of pages");
        return Err(());
    }

    if n == 0 {
        pr_err!("Cannot allocate 0 pages");
        return Err(());
    }

    // Handle not enough of free blocks.
    let free_pages = mm.max_pages - mm.used_pages;

    if n >= free_pages {
        pr_err!("Page count exceed total number of free pages");
        return Err(());
    }

    let start_pos = get_free_pages(&mm, count)?;
    let addr      = page_num_to_phys(start_pos);

    // Set page to zero.
    unsafe {
        ptr::write_bytes(addr as *mut c_void, 0, n << PAGE_SHIFT);
    }

    // Set n pages as used.
    for i in 0..n {
        mm.bitmap.set(start_pos + i);
    }
    mm.used_pages += n;

    pr_debug!("Allocated {} pages at address <{:#010X}>", n, addr);
    Ok(addr)
}

/// Free given pages.
///
/// # Parameters
/// - `count` - given number of pages to free.
///
/// # Returns
/// Ok  - in case of success.
/// Err - otherwise.
pub fn free_pages(addr: u32, count: u32) -> Result<(), ()> {
    let mut mm = manager::MM.lock();
    let n      = count as usize;

    // Handle incorrect number of pages.
    if n >= mm.max_pages {
        pr_err!("Page count exceed total number of pages");
        return Err(());
    }

    if n >= mm.used_pages {
        pr_err!("Page count exceed total number of used pages");
        return Err(());
    }

    // It is forbidden to free these pages, because they are
    // contain GDT & multiboot info structure.
    let begin_pos = phys_to_page_num(addr);
    let end_pos   = phys_to_page_num(addr);
    let range     = begin_pos..end_pos;

    if range.contains(&0) || range.contains(&16) {
        pr_err!("Page count is in forbidden range {:#?}", range);
        return Err(());
    }

    // Set n pages as free.
    for i in 0..n {
        mm.bitmap.unset(begin_pos + i);
    }
    mm.used_pages -= n;

    pr_debug!("Freed {} pages at address <{:#010X}>", n, addr);
    Ok(())
}

use crate::tests::*;

exotest! {
    exotest_test_cases! {
        test_successful_page_allocation, {
            let count  = 1;
            let result = alloc_pages(count);

            assert!(result.is_ok());

            let addr = result.unwrap();
            let _    = free_pages(addr, count);
        },

        test_zero_page_allocation, {
            let count  = 0;
            let result = alloc_pages(count);

            assert!(result.is_err());
        },

        test_super_large_allocation, {
            let count  = 65536;
            let result = alloc_pages(count);

            assert!(result.is_err());
        },

        test_successful_large_page_allocation, {
            let count  = 2048;
            let result = alloc_pages(count);

            assert!(result.is_ok());

            let addr = result.unwrap();
            let _    = free_pages(addr, count);
        },

        test_multiple_page_allocations, {
            // Allocate first sequence of pages.
            let count1  = 1;
            let result1 = alloc_pages(count1);

            assert!(result1.is_ok());

            // Allocate second sequence of pages.
            let count2  = 2;
            let result2 = alloc_pages(count2);

            assert!(result2.is_ok());

            // Check that these pages addresses are close to each other.
            let addr1 = result1.unwrap();
            let addr2 = result2.unwrap();

            assert_eq!(addr2 - addr1, PAGE_SIZE as u32);

            let _ = free_pages(addr1, count1);
            let _ = free_pages(addr2, count2);
        }
    }
}