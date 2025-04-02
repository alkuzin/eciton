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

//! Declares bitmap data structure.

use crate::bitops::{bits_per_type, bytes_to_bits};
use core::{
    ops::{BitAnd, BitAndAssign, BitOrAssign, Not, Shl},
    cmp::PartialEq, fmt::Debug, ptr::null_mut
};

/// Bitmap item trait.
pub trait BitmapItem: From<u8> + Copy + BitAnd<Output = Self> + BitAndAssign
+ Shl<usize, Output = Self> + BitOrAssign + Not<Output = Self> + PartialEq<Self>
+ Debug {}

// Restrict bitmap element to some unsigned values.
impl BitmapItem for u8 {}
impl BitmapItem for u16 {}
impl BitmapItem for u32 {}
impl BitmapItem for u64 {}

/// Bitmap data enumeration.
#[derive(Debug)]
pub enum BitmapData<T: BitmapItem, const COUNT: usize> {
    /// Used to construct bitmap from raw pointer.
    Pointer(*mut T),
    /// Used to construct bitmap from an array.
    Array([T;COUNT]),
}

/// Bitmap constructed with raw pointer by default.
impl<T: BitmapItem, const COUNT: usize> Default for BitmapData<T, COUNT> {
    fn default() -> Self {
        Self::Pointer(null_mut())
    }
}

/// Bitmap data structure.
///
/// # Parameters
/// - `T` - given type of the elements stored in the bitmap.
/// Highly recommended to use on of these types: `u8`, `u16`, `u32` or `u64`.
/// - `COUNT` - given bitmap data array elements count.
/// If bitmap is constructed from an array pointer - use `COUNT = 0`.
#[derive(Debug, Default)]
pub struct Bitmap<T: BitmapItem, const COUNT: usize> {
    /// Storing bitmap contents.
    pub data: BitmapData<T, COUNT>,
    /// Size of data in bytes.
    pub size: usize,
    /// Total number of bits in data.
    pub bits: usize,
}

impl<T: BitmapItem, const COUNT: usize> Bitmap<T, COUNT> {
    /// Construct new bitmap object from array pointer.
    ///
    /// # Parameters
    /// - `data` - given data pointer to set.
    /// - `size` - given size of data in bytes.
    ///
    /// # Returns
    /// - New Bitmap object.
    pub fn from_pointer(data: *mut T, size: usize) -> Self {
        Bitmap {
            data: BitmapData::Pointer(data),
            bits: bytes_to_bits(size),
            size,
        }
    }

    /// Construct new bitmap object from an array.
    ///
    /// # Parameters
    /// - `data` - given array to set.
    ///
    /// # Returns
    /// - New Bitmap object.
    pub fn from_array(data: [T;COUNT]) -> Self {
        let size = COUNT * size_of::<T>();

        Bitmap {
            data: BitmapData::Array(data),
            bits: bytes_to_bits(size),
            size,
        }
    }

    /// Get bitmap data as a raw pointer.
    ///
    /// # Returns
    /// - Bitmap data as a raw pointer - in case of success.
    /// - `None` - if bitmap constructed from an array.
    pub fn as_pointer(&self) -> Option<*mut T> {
        match self.data {
            BitmapData::Pointer(ptr) => Some(ptr),
            BitmapData::Array(_)     => None,
        }
    }

    /// Get bitmap data as an array.
    ///
    /// # Returns
    /// - Bitmap data as an array - in case of success.
    /// - `None` - if bitmap constructed from raw pointer.
    pub fn as_array(&self) -> Option<&[T;COUNT]> {
        match self.data {
            BitmapData::Array(ref array) => Some(array),
            BitmapData::Pointer(_)       => None,
        }
    }

    /// Get data index.
    ///
    /// # Parameters
    /// - `pos` - given bit position.
    ///
    /// # Returns
    /// Index in data corresponding to `pos`.
    #[inline(always)]
    fn index(&self, pos: usize) -> usize {
        pos / self.bits_per_element()
    }

    /// Get position bitmask.
    ///
    /// # Parameters
    /// - `pos` - given bit position.
    ///
    /// # Returns
    /// Bitmask corresponding to `pos`.
    #[inline(always)]
    fn bitmask(&self, pos: usize) -> T {
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
        let index   = self.index(pos);
        let bitmask = self.bitmask(pos);

        let ptr: *const T = match self.data {
            BitmapData::Pointer(ptr) => ptr,
            BitmapData::Array(arr)   => arr.as_ptr(),
        };

        unsafe {
            (*ptr.add(index) & bitmask) != 0.into()
        }
    }

    /// Set specific bit.
    ///
    /// # Parameters
    /// - `pos` - given bit position.
    #[inline(always)]
    pub fn set(&mut self, pos: usize) {
        let index   = self.index(pos);
        let bitmask = self.bitmask(pos);

        let ptr: *mut T = match self.data {
            BitmapData::Pointer(ref ptr)   => *ptr,
            BitmapData::Array(ref mut arr) => arr.as_mut_ptr(),
        };

        unsafe {
            *ptr.add(index) |= bitmask
        }
    }

    /// Unset specific bit.
    ///
    /// # Parameters
    /// - `pos` - given bit position.
    #[inline(always)]
    pub fn unset(&self, pos: usize) {
        let index   = self.index(pos);
        let bitmask = self.bitmask(pos);

        let ptr: *mut T = match self.data {
            BitmapData::Pointer(ptr)   => ptr,
            BitmapData::Array(mut arr) => arr.as_mut_ptr(),
        };

        unsafe {
            *ptr.add(index) &= !bitmask
        }
    }

    /// Get number of bits per element.
    ///
    /// # Returns
    /// Number of bits per element.
    #[inline(always)]
    pub fn bits_per_element(&self) -> usize {
        bits_per_type::<T>()
    }

    /// Get number of elements.
    ///
    /// # Returns
    /// Number of elements in data.
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.bits.div_ceil(self.bits_per_element())
    }

}