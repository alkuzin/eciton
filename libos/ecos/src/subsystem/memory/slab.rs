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

//! SLAB allocator slab declarations.

use eciton_sdk::{bitops::bits_per_type, collections::Bitmap};
use crate::subsystem::memory::Page;

/// Constant representing used object in bitmap.
const OBJECT_USED: bool = true;

/// Constant representing free object in bitmap.
const OBJECT_FREE: bool = false;

/// Max number of objects that can be managed by bitmap.
const BITMAP_MAX_OBJECTS: usize = 256;

/// Bitmap array size (for u32).
const BITMAP_COUNT: usize = BITMAP_MAX_OBJECTS / bits_per_type::<u32>();

/// Slab structure.
#[derive(Debug, Default)]
pub struct Slab {
    /// Bitmap for managing slab objects.
    bitmap: Bitmap<u32, BITMAP_COUNT>,
    /// First object memory address.
    pub s_mem: u32,
    /// Number of active objects in the slab.
    pub inuse: u32,
    /// Total number of objects.
    pub objnum: u32,
    /// Size of each object.
    pub objsize: u32,
}

impl Slab {
    /// Construct new Slab object.
    ///
    /// # Parameters
    /// - `addr` - given memory page begin address.
    ///
    /// # Returns
    /// - New `Slab` object.
    pub fn new(addr: u32) -> Self {
        Self {
            bitmap:  Bitmap::from_array([0u32;BITMAP_COUNT]),
            s_mem:   addr,
            objsize: 0,
            objnum:  0,
            inuse:   0,
        }
    }

    /// Set object related data.
    ///
    /// # Parameters
    /// - `objnum`  - given total number of objects.
    /// - `objsize` - given size of each object.
    #[inline(always)]
    pub fn set(&mut self, objnum: u32, objsize: u32) {
        self.objsize = objsize;
        self.objnum  = objnum;
    }

    /// Check if slab is full.
    ///
    /// # Returns
    /// - `true`  - if slab is full.
    /// - `false` - otherwise.
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.objnum == self.inuse
    }

    /// Find free object in slab.
    ///
    /// # Returns
    /// - Free object index - in case of success.
    /// - `Err(msg)` - error with message `msg` - otherwise.
    fn find_free_object(&self) -> Result<usize, &'static str> {
        let freelist         = &self.bitmap;
        let bits_per_element = freelist.bits_per_element();
        let capacity         = self.objnum as usize / bits_per_element + 1;
        let freelist_arr     = freelist.as_array().unwrap();

        #[allow(unused_assignments)]
        let mut pos: usize = 0;

        for i in 0..capacity {
            // Skip groups of used pages.
            let group = freelist_arr[i];

            if group != 0xFFFFFFFF {
                // Handle each group.
                for j in 0..bits_per_element {
                    pos = bits_per_element * i + j;

                    // Skip until free page.
                    while freelist.get(pos) == OBJECT_USED {
                        pos += 1;
                    }

                    if freelist.get(pos) == OBJECT_FREE {
                        return Ok(pos);
                    }
                }
            }
        }

        Err("Cannot find free object to allocate.")
    }

    /// Allocate single object.
    ///
    /// # Returns
    /// - Allocated object memory address - in case of success.
    /// - `Err(msg)` - error with message `msg` - otherwise.
    pub fn alloc_object(&mut self) -> Result<u32, &'static str> {
        // Try to find free object.
        let free_object_index = self.find_free_object()?;

        // Check if slab is full.
        if self.is_full() {
            return Err("Slab is full.");
        }

        // Calculate object offset.
        let object = self.s_mem + free_object_index as u32 * self.objsize;

        // Mark object as used.
        self.bitmap.set(free_object_index);
        self.inuse += 1;

        Ok(object)
    }

    /// Free single object.
    ///
    /// # Parameters
    /// - `addr` - given object memory address.
    ///
    /// # Returns
    /// - `Ok(flag)` - `flag` whether to free slab - in case of success.
    /// - `Err(msg)` - error with message `msg` - otherwise.
    pub fn free_object(&mut self, addr: u32) -> Result<bool, &'static str> {
        let end_mem    = self.s_mem + Page::size() as u32;
        let object_pos = end_mem - addr / self.objsize;

        // Mark object as free.
        self.bitmap.unset(object_pos as usize);
        self.inuse -= 1;

        // Check if slab empty after freeing of the object.
        if self.inuse == 0 {
            return Ok(true)
        }

        Ok(false)
    }
}