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

use eciton_sdk::{collections::Bitmap, bitops::bits_to_bytes, page::Page};
use crate::{pr_debug, BOOT_INFO};
use core::{ffi::c_void, ptr};
use manager::MemoryManager;
use layout::*;

/// Constant representing free object in bitmap.
const PAGE_FREE: bool = false;

/// Constant representing used object in bitmap.
const PAGE_USED: bool = true;

/// Max number of pages to allocate/free at once.
pub const PAGE_LIMIT: usize = 128;

/// Number of page containing GDT.
const GDT_PAGE_NUM: usize = 0;

/// Number of page containing multiboot info structure.
const MULTIBOOT_PAGE_NUM: usize = 16;

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
    mm.mark_as_used(kernel_begin_paddr(), kernel_size as usize + Page::size());

    // Mark memory between kernel end & bitmap as used.
    mm.mark_as_used(bitmap_addr.wrapping_sub(STACK_SIZE) as u32, STACK_SIZE);

    // Mark bitmap memory as used.
    mm.mark_as_used(bitmap_data as u32, bitmap_size);

    // These pages containing reserved data that should not
    // be accessed, so it was set as used:
    mm.reserve_page(GDT_PAGE_NUM);
    mm.reserve_page(MULTIBOOT_PAGE_NUM);

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

    let free_pages = mm.max_pages - mm.used_pages;

    // Handle incorrect number of pages.
    if n >= mm.max_pages || n == 0 || n >= PAGE_LIMIT || n >= free_pages {
        return Err(());
    }

    let start_pos = get_free_pages(&mm, count)?;
    let addr      = Page::addr_from(start_pos);

    // Set page to zero.
    unsafe {
        ptr::write_bytes(addr as *mut c_void, 0, n << Page::shift());
    }

    // Set n pages as used.
    for i in 0..n {
        mm.reserve_page(start_pos + i)
    }

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
    if n == 0 || n >= PAGE_LIMIT || n >= mm.max_pages || n >= mm.used_pages {
        return Err(());
    }

    // It is forbidden to free these pages, because they are
    // contain GDT & multiboot info structure.
    let begin_pos = Page::page_num_from(addr);
    let end_pos   = Page::page_num_from(addr + (count << Page::shift()));
    let range     = begin_pos..end_pos;

    if range.contains(&0) || range.contains(&16) {
        return Err(());
    }

    // Handle freeing of already free page.
    for i in 0..n {
        if mm.bitmap.get(begin_pos + i) == PAGE_FREE {
            return Err(());
        }
    }

    // Set n pages as free.
    for i in 0..n {
        mm.bitmap.unset(begin_pos + i);
    }
    mm.used_pages -= n;

    pr_debug!("Freed {} pages at address <{:#010X}>", n, addr);
    Ok(())
}

use crate::test::*;

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
            let count  = (PAGE_LIMIT - 1) as u32;
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

            assert_eq!(addr2 - addr1, Page::size() as u32);

            let _ = free_pages(addr1, count1);
            let _ = free_pages(addr2, count2);
        },

        test_allocating_more_pages_than_can_be_allocated, {
            let result = alloc_pages(PAGE_LIMIT as u32);

            assert!(result.is_err());
        },

        test_successful_freeing, {
            let count  = 1;
            let result = alloc_pages(count);

            assert!(result.is_ok());

            let addr   = result.unwrap();
            let result = free_pages(addr, count);

            assert!(result.is_ok());
        },

        test_freeing_zero_pages, {
            let count  = 0;
            let result = free_pages(0x1000, count);

            assert!(result.is_err());
        },

        test_freeing_free_page, {
            let count  = 1;
            let result = alloc_pages(count);

            assert!(result.is_ok());

            let addr   = result.unwrap();
            let result = free_pages(addr, count);

            assert!(result.is_ok());

            // Try to free already freed page.
            let result = free_pages(addr, count);

            assert!(result.is_err());
        },

        test_freeing_non_allocated_pages, {
            let result = free_pages(0, 10);

            assert!(result.is_err());
        },

        test_freeing_more_pages_than_can_be_freed, {
            let result = free_pages(0x1000, PAGE_LIMIT as u32);

            assert!(result.is_err());
        },

        test_freeing_forbidden_pages, {
            let count  = 1;
            let addr   = Page::addr_from(GDT_PAGE_NUM);
            let result = free_pages(addr, count);

            assert!(result.is_err());

            let addr   = Page::addr_from(MULTIBOOT_PAGE_NUM);
            let result = free_pages(addr, count);

            assert!(result.is_err());
        }
    }
}