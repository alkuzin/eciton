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

//! Memory management subsystem declaration.

pub mod cache;
pub mod slab;
pub mod page;

pub use cache::*;
pub use page::*;
pub use slab::*;
use eciton_sdk::{
    math::roundup_pow_of_two,
    bitops::bits_per_type,
    collections::Bitmap,
    MemoryUnit
};
use crate::{
    subsystem::{Subsystem, SubsystemResult},
    api::exo::{allocpg, freepg, AllocUnit},
    pr_debug
};
use core::slice;

/// Constant representing used slab in bitmap.
const SLAB_USED: bool = true;

/// Constant representing free slab in bitmap.
const SLAB_FREE: bool = false;

/// Total number of caches used by SLAB allocator.
const CACHE_COUNT: usize = 9;

/// Number of pages allocating for Slab structs array.
const SLABS_PAGES: usize = 1;

/// Total number of slabs.
const SLABS_COUNT: usize = SLABS_PAGES * Page::size() / size_of::<Slab>();

/// Bitmap array size (for u32).
const BITMAP_COUNT: usize = calculate_bitmap_count();

/// Calculate bitmap array size (for u32)
///
/// # Returns
/// - Bitmap array count.
const fn calculate_bitmap_count() -> usize {
    let bits_per_element = bits_per_type::<u32>();

    if SLABS_COUNT.is_power_of_two() {
        SLABS_COUNT / bits_per_element
    }
    else {
        (roundup_pow_of_two(SLABS_COUNT) / bits_per_element) - 1
    }
}

/// SLAB allocator struct.
#[derive(Debug, Default)]
pub struct SlabAllocator<'a> {
    /// Bitmap for managing slabs.
    bitmap: Bitmap<u32, BITMAP_COUNT>,
    /// Array of predefined caches.
    caches: [Cache;CACHE_COUNT],
    /// Array of slabs.
    slabs: Option<&'a mut [Slab]>,
    /// Allocation unit for array of slabs structs.
    slabs_arr_alloc_unit: Option<AllocUnit>,
    /// Allocation unit for all slabs.
    all_slabs_alloc_unit: Option<AllocUnit>,
}

impl Subsystem for SlabAllocator<'_> {
    /// Initialize SLAB allocator.
    ///
    /// # Returns
    /// - `Ok`       - in case of success.
    /// - `Err(msg)` - error message otherwise.
    fn init(&mut self) -> SubsystemResult {
        // Allocate pages for slab array.
        if let Ok(alloc_unit) = allocpg(SLABS_PAGES as u32) {
            self.slabs_arr_alloc_unit = Some(alloc_unit);
            let slabs_ptr = alloc_unit.addr as *mut Slab;

            self.slabs = Some(unsafe {
                slice::from_raw_parts_mut(slabs_ptr, SLABS_COUNT)
            });

            // Allocate pages for all slabs.
            if let Ok(alloc_unit) = allocpg(SLABS_COUNT as u32) {
                self.all_slabs_alloc_unit = Some(alloc_unit);
                let mut page_addr = alloc_unit.addr;

                for slab in self.slabs.as_mut().unwrap().iter_mut() {
                    *slab = Slab::new(page_addr);
                    page_addr += Page::size() as u32;
                }
            }
            else {
                return Err("Error to allocate memory pages for all slabs.");
            }

            let slabs = *self.slabs.as_mut().unwrap() as *mut [Slab];

            // Initialize slab caches.
            self.caches = [
                Cache::new("kmalloc-8", 8, slabs),
                Cache::new("kmalloc-16", 16, slabs),
                Cache::new("kmalloc-32", 32, slabs),
                Cache::new("kmalloc-64", 64, slabs),
                Cache::new("kmalloc-128", 128, slabs),
                Cache::new("kmalloc-256", 256, slabs),
                Cache::new("kmalloc-512", 512, slabs),
                Cache::new("kmalloc-1k", 1.kb(), slabs),
                Cache::new("kmalloc-2k", 2.kb(), slabs),
            ];

            return Ok(());
        }

        Err("Error to allocate memory pages for SLAB allocator.")
    }

    /// Run SLAB allocator.
    ///
    /// # Returns
    /// - `Ok`       - in case of success.
    /// - `Err(msg)` - error message otherwise.
    fn run(&mut self) -> SubsystemResult {
        // Print SLAB allocator debug info.
        pr_debug!("Slab size:    {} bytes.", size_of::<Slab>());
        pr_debug!("Total slabs:  {}.", self.slabs.as_ref().unwrap().len());
        pr_debug!("Total caches: {}.", self.caches.len());

        for cache in &self.caches {
            pr_debug!(
                "Created cache: [ {:<12} objsize: {:<5} objnum: {:<4}]",
                cache.name, cache.objsize, cache.objnum
            );
        }

        Ok(())
    }

    /// Free all allocated memory for SLAB allocator.
    ///
    /// # Returns
    /// - `Ok`       - in case of success.
    /// - `Err(msg)` - error message otherwise.
    fn exit(&mut self) -> SubsystemResult {
        if freepg(self.slabs_arr_alloc_unit.unwrap()).is_err() {
            return Err("Error to free memory pages allocated for slab array.")
        }

        if freepg(self.all_slabs_alloc_unit.unwrap()).is_err() {
            return Err("Error to free memory pages allocated for all slabs.")
        }

        self.slabs = None;
        Ok(())
    }

    /// Get subsystem name.
    ///
    /// # Returns
    /// - Subsystem name in string representation.
    fn name(&self) -> &'static str {
        "SLAB allocator"
    }
}

impl<'a> SlabAllocator<'a> {
    /// Get the specific cache index in cache array.
    ///
    /// # Parameters
    /// - `size` - given size of memory block to allocate.
    ///
    /// # Returns
    /// - Cache index in cache array.
    fn get_cache_index(&self, size: usize) -> usize {
        let mut rounded = roundup_pow_of_two(size);
        let mut index   = 0;

        while rounded > 8 {
            rounded >>= 1;
            index += 1;
        }

        index
    }

    /// Find free slab index.
    ///
    /// # Returns
    /// - Index in array of slabs - in case of success.
    /// - `Err(msg)` - error message - otherwise.
    fn find_free_slab(&self) -> Result<usize, &'static str> {
        let bitmap           = &self.bitmap;
        let bits_per_element = bitmap.bits_per_element();
        let capacity         = self.slabs.as_ref().unwrap().len();

        #[allow(unused_assignments)]
        let mut pos: usize = 0;

        for i in 0..capacity {
            // Skip groups of used pages.
            let group = unsafe { *bitmap.as_pointer().unwrap().add(i) };

            if group != 0xFFFFFFFF {
                // Handle each group.
                for j in 0..bits_per_element {
                    pos = bits_per_element * i + j;

                    // Skip until free page.
                    while bitmap.get(pos) == SLAB_USED {
                        pos += 1;
                    }

                    if bitmap.get(pos) == SLAB_FREE {
                        return Ok(pos);
                    }
                }
            }
        }

        Err("Out of slabs.")
    }

    /// Allocate single object.
    ///
    /// # Parameters
    /// - `idx` - given cache index.
    ///
    /// # Returns
    /// - `Object start memory address` - in case of success.
    /// - `Err(msg)` - error message otherwise.
    pub fn alloc_object(&mut self, idx: usize) -> Result<u32, &'static str> {
        if let None = self.slabs {
            return Err("Error to allocate object. Slabs array is none.");
        }

        let free_slab_index = self.find_free_slab()?;
        let cache           = &mut self.caches[idx];

        if cache.is_full() || cache.is_empty() {
            // Assign free slab to cache.
            cache.push(free_slab_index)?;

            // Mark assigned slab as used.
            self.bitmap.set(free_slab_index);
        }

        let addr = cache.alloc()?;
        Ok(addr)
    }

    /// Free single object.
    ///
    /// # Parameters
    /// - `addr` - given object memory address.
    /// - `idx`  - given cache index.
    ///
    /// # Returns
    /// - `Ok`       - in case of success.
    /// - `Err(msg)` - error message otherwise.
    pub fn free_object(&mut self, addr: u32, idx: usize) -> Result<(), &'static str> {
        if let None = self.slabs {
            return Err("Error to free object. Slabs array is none.");
        }

        // Check that address is within memory allocated for slabs.
        let alloc_unit  = self.all_slabs_alloc_unit.as_ref().unwrap();
        let size        = alloc_unit.count * Page::size() as u32;
        let lower_range = alloc_unit.addr;
        let upper_range = lower_range + size;
        let range       = lower_range..upper_range;

        if !range.contains(&addr) {
            return Err("Error to free object. Incorrect memory address.")
        }

        let cache = &mut self.caches[idx];

        if cache.is_empty() {
           return Err("Error to free object. Cache is empty.")
        }

        let pos  = (addr - lower_range) as usize / Page::size();
        let flag = cache.free(addr, pos)?;

        // Free slab if needed.
        if flag {
            self.bitmap.unset(pos);
        }

        Ok(())
    }
}