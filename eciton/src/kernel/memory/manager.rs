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

//! Kernel physical memory manager module.

use crate::{
    kernel::multiboot::{
        MultibootInfo,
        MultibootMemoryType,
        MultibootMmapEntry
    }, pr_debug
};

use super::{phys_to_page_num, PAGE_SHIFT};
use eciton_sdk::collections::Bitmap;
use lazy_static::lazy_static;
use spin::Mutex;

/// Physical memory manager struct.
#[derive(Debug, Default)]
pub struct MemoryManager {
    /// Total available memory.
    pub mem_available: usize,
    /// Total physical memory.
    pub mem_total: usize,
    /// Total number of pages.
    pub max_pages: usize,
    /// Number of used pages.
    pub used_pages: usize,
    /// Physical memory map.
    pub bitmap: Bitmap<u32, 0>,
    // TODO: add free_pages: usize,
}

impl MemoryManager {
    /// Get information about memory regions.
    ///
    /// # Parameters
    /// - `boot_info` - given multiboot info structure.
    pub fn detect_memory(&mut self, boot_info: &MultibootInfo) {
        pr_debug!("BIOS-provided physical RAM map:");
        let mut i: usize = 0;

        while i < boot_info.mmap_length as usize {
            let ptr      = boot_info.mmap_addr + i as u32;
            let mmmt     = unsafe { *(ptr as *const MultibootMmapEntry) };
            let mem_type = mmmt.mtype;

            if let MultibootMemoryType::Available = mem_type {
                self.mem_available += mmmt.len as usize
            };

            self.mem_total += mmmt.len as usize;

            i += size_of::<MultibootMmapEntry>();

            let begin = mmmt.addr;
            let end   = begin + mmmt.len - 1;
            pr_debug!("[mem {:#010X}-{:#010X}] {:?}.", begin, end, mem_type);
        }

        self.max_pages = self.mem_total >> PAGE_SHIFT;
    }

    /// Mark memory region as free.
    ///
    /// # Parameters
    /// - `addr` - given base address of the region.
    /// - `size` - given size of the region in bytes.
    pub fn mark_as_free(&mut self, addr: u32, size: usize) {
        let mut pos = phys_to_page_num(addr);
        let mut n   = size >> PAGE_SHIFT;

        while n > 0 {
            self.bitmap.unset(pos);
            self.used_pages -= 1;
            pos += 1;
            n -= 1;
        }
    }

    /// Mark memory region as used.
    ///
    /// # Parameters
    /// - `addr` - given base address of the region.
    /// - `size` - given size of the region in bytes.
    pub fn mark_as_used(&mut self, addr: u32, size: usize) {
        let mut pos = phys_to_page_num(addr);
        let mut n   = size >> PAGE_SHIFT;

        while n > 0 {
            self.bitmap.set(pos);
            self.used_pages += 1;
            pos += 1;
            n -= 1;
        }
    }

    /// Reserve specific page.
    ///
    /// # Parameters
    /// - `n` - given page number.
    pub fn reserve_page(&mut self, n: usize) {
        self.bitmap.set(n);
        self.used_pages += 1;
    }

    /// Free all available memory regions.
    ///
    /// # Parameters
    /// - `boot_info` - given multiboot info structure.
    pub fn free_available_memory(&mut self, boot_info: &MultibootInfo) {
        let mut i: usize = 0;

        while i < boot_info.mmap_length as usize {
            let ptr      = boot_info.mmap_addr + i as u32;
            let mmmt     = unsafe { *(ptr as *const MultibootMmapEntry) };
            let mem_type = mmmt.mtype;

            if let MultibootMemoryType::Available = mem_type {
                self.mark_as_free(mmmt.addr as u32, mmmt.len as usize);
            };

            i += size_of::<MultibootMmapEntry>();
        }
    }
}

unsafe impl Sync for MemoryManager {}
unsafe impl Send for MemoryManager {}

lazy_static! {
    pub static ref MM: Mutex<MemoryManager> = Mutex::new(MemoryManager::default());
}