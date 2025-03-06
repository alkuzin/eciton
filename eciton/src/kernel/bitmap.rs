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

//! Declares bitmap structure.

use super::bitops::{bits_per_type, bytes_to_bits};
use core::{
    cmp::PartialEq,
    ops::{BitAnd, BitAndAssign, BitOrAssign, Not, Shl},
    ptr
};

/// Bitmap data structure.
#[derive(Debug)]
pub struct Bitmap<T> {
    /// Data pointer.
    pub data: *mut T,
    /// Size of data in bytes.
    pub size: usize,
    /// Total number of bits in data.
    pub bits: usize,
}

impl<T> Bitmap<T>
where
    T: From<u8> + Copy + BitAnd<Output = T> + Shl<usize, Output = T>
    + BitOrAssign + Not<Output = T> + BitAndAssign + PartialEq<u32>
{
    /// Construct new bitmap object.
    ///
    /// # Parameters
    /// - `data` - given data pointer to set.
    /// - `size` - given size of data in bytes.
    pub fn new(data: *mut T, size: usize) -> Self {
        Bitmap { data, size, bits: bytes_to_bits(size) }
    }

    /// Get data index.
    ///
    /// # Parameters
    /// - `pos` - given bit position.
    ///
    /// # Returns
    /// Index in data corresponding to `pos`.
    #[inline(always)]
    fn index(pos: usize) -> usize {
        pos / Self::bits_per_element()
    }

    /// Get position bitmask.
    ///
    /// # Parameters
    /// - `pos` - given bit position.
    ///
    /// # Returns
    /// Bitmask corresponding to `pos`.
    #[inline(always)]
    fn bitmask(pos: usize) -> T {
        T::from(1u8) << (pos % bits_per_type::<T>())
    }

    /// Get bit value.
    ///
    /// # Parameters
    /// - `pos` - given bit position.
    ///
    /// # Returns
    /// - `true`  - if bit is set.
    /// - `false` - otherwise.
    #[inline(always)]
    pub fn get(&self, pos: usize) -> bool {
        let index   = Self::index(pos);
        let bitmask = Self::bitmask(pos);

        unsafe {
            (*self.data.add(index) & bitmask) != 0
        }
    }

    /// Set specific bit.
    ///
    /// # Parameters
    /// - `pos` - given bit position.
    #[inline(always)]
    pub fn set(&self, pos: usize) {
        let index   = Self::index(pos);
        let bitmask = Self::bitmask(pos);

        unsafe {
            (*self.data.add(index) |= bitmask);
        }
    }

    /// Unset specific bit.
    ///
    /// # Parameters
    /// - `pos` - given bit position.
    #[inline(always)]
    pub fn unset(&self, pos: usize) {
        let index   = Self::index(pos);
        let bitmask = Self::bitmask(pos);

        unsafe {
            (*self.data.add(index) &= !bitmask);
        }
    }

    /// Get number of bits per element.
    ///
    /// # Returns
    /// Number of bits per element.
    #[inline(always)]
    pub fn bits_per_element() -> usize {
        bits_per_type::<T>()
    }

    /// Get number of elements.
    ///
    /// # Returns
    /// Number of elements in data.
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.bits.div_ceil(Self::bits_per_element())
    }

}

/// Bitmap default value implementation.
impl<T> Default for Bitmap<T> {
    fn default() -> Self {
        Bitmap { data: ptr::null_mut(), size: 0, bits: 0 }
    }
}