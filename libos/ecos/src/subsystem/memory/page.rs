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

//! Memory page declarations.

/// Memory page structure.
#[derive(Debug, Default)]
pub struct Page {
    /// Page frame number (position in memory map).
    pfn: usize,
    /// Page status.
    flags: u8,
}

impl Page {
    /// Construct new Page object.
    ///
    /// # Parameters
    /// - `addr`  - given page physical address.
    /// - `flags` - given page flags.
    ///
    /// # Returns
    /// New Page object.
    pub fn new(addr: u32, flags: u8) -> Self {
        let pfn = Self::page_num_from(addr);
        Self { pfn, flags }
    }

    /// Get page memory address.
    ///
    /// # Returns
    /// - Page memory address.
    #[inline(always)]
    pub fn addr(&self) -> u32 {
        Self::addr_from(self.pfn)
    }

    /// Get memory page size in bytes.
    ///
    /// # Returns
    /// - Memory page size in bytes.
    #[inline(always)]
    pub const fn size() -> usize {
        4096
    }

    /// Get memory page bit shift.
    ///
    /// # Returns
    /// - Memory page bit shift.
    #[inline(always)]
    pub const fn shift() -> u8 {
        0xC
    }

    /// Convert physical address to page frame number.
    ///
    /// # Parameters
    /// - `addr` - given page physical address.
    ///
    /// # Returns
    /// Page frame number.
    #[inline(always)]
    pub fn page_num_from(addr: u32) -> usize {
        addr as usize >> Self::shift()
    }

    /// Convert page frame number to physical address.
    ///
    /// # Parameters
    /// - `addr` - given page frame number.
    ///
    /// # Returns
    /// Page frame physical address.
    #[inline(always)]
    pub fn addr_from(pfn: usize) -> u32 {
        (pfn << Self::shift()) as u32
    }
}
