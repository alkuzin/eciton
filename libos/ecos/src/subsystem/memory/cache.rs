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

//! SLAB allocator cache declarations.

use eciton_sdk::{
    math::{ceil, log2, roundup_pow_of_two},
    collections::StaticList
};
use super::{Page, Slab};

/// Max limit of slabs per each cache.
const SLABS_PER_CACHE: usize = 10;

/// Memory cache struct.
#[derive(Debug, Default)]
pub struct Cache {
    /// Pointer to array of slabs.
    slabs: Option<*mut [Slab]>,
    /// Slab indexes list.
    indexes: StaticList<usize, SLABS_PER_CACHE>,
    /// Slab list head.
    head: Option<usize>,
    /// Next free slab index.
    next_free: Option<usize>,
    /// Size of slab in pages (2^gfporder).
    gfporder: u32,
    /// Size of object.
    pub objsize: u32,
    /// Number of objects in each slab.
    pub objnum: u32,
    /// Cache name.
    pub name: &'static str,
}

impl Cache {
    /// Construct a new Cache object.
    ///
    /// # Parameters
    /// - `name`  - given cache name.
    /// - `size`  - given size of cache objects.
    /// - `slabs` - given pointer to array of slabs.
    ///
    /// # Returns
    /// - New Cache object.
    pub fn new(name: &'static str, size: usize, slabs: *mut [Slab]) -> Self {
        let tmp      = roundup_pow_of_two(size) as f64;
        let objsize  = tmp as u32;
        let gfporder = ceil(log2(tmp)) as u32;
        let objnum   = (Page::size() >> gfporder) as u32;

        Self {
            indexes:   Default::default(),
            slabs:     Some(slabs),
            head:      None,
            next_free: None,
            objsize,
            gfporder,
            objnum,
            name
        }
    }

    /// Check if cache is empty.
    ///
    /// # Returns
    /// - `true`  - if cache is empty.
    /// - `false` - otherwise.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Check if cache is full.
    ///
    /// # Returns
    /// - `true`  - if cache is full.
    /// - `false` - otherwise.
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        if self.next_free.is_none() {
            false
        }
        else {
            let next_free_index = self.next_free.unwrap();
            let next_free_slab  = self.get_slab(next_free_index);

            next_free_slab.is_full()
        }
    }

    /// Push free slab to cache.
    ///
    /// # Parameters
    /// - `pos` - given slab position in slab array.
    ///
    /// # Returns
    /// - `Ok`       - in case of success.
    /// - `Err(msg)` - error with message `msg` - otherwise.
    pub fn push(&mut self, pos: usize) -> Result<(), &'static str> {
        let slab = self.get_slab(pos);
        slab.set(self.objnum, self.objsize);

        self.indexes.push(pos)?;
        self.next_free = Some(pos);

        if self.is_empty() {
            self.head = Some(pos);
        }

        Ok(())
    }

    /// Get specific slab.
    ///
    /// # Parameters
    /// - `pos` - given slab position in slab array.
    ///
    /// # Returns
    /// - Slab reference.
    #[inline(always)]
    fn get_slab(&self, pos: usize) -> &'_ mut Slab {
        unsafe {
            let slabs: &mut [Slab] = &mut *self.slabs.unwrap();
            let slab = &mut slabs[pos];
            slab
        }
    }

    /// Allocate free object from cache.
    ///
    /// # Returns
    /// - Object memory address - in case of success.
    /// - `Err(msg)` - error with message `msg` - otherwise.
    pub fn alloc(&mut self) -> Result<u32, &'static str> {
        let pos  = self.indexes.tail().unwrap().value;
        let slab = self.get_slab(pos);
        let addr = slab.alloc_object()?;

        Ok(addr)
    }
}