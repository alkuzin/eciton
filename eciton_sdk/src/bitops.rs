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

//! Contain bit operations declarations.

use core::{
    cmp::PartialOrd,
    mem::size_of,
    ops::{BitAnd, BitAndAssign, BitOrAssign, Not, Shl},
};

/// Number of bits per byte.
pub const BITS_PER_BYTE: usize = 8;

/// Convert bits to bytes.
///
/// # Parameters
/// - `n` - given number of bits.
///
/// # Returns
/// Number of bytes needed for containing `n` bits.
#[inline(always)]
pub const fn bits_to_bytes(n: usize) -> usize {
    // Assuming that a byte contains 8 bits.
    (n + 7) >> 0x3
}

/// Convert bytes to bits.
///
/// # Parameters
/// - `n` - given number of bytes.
///
/// # Returns
/// Number of bits in `n` bytes.
#[inline(always)]
pub const fn bytes_to_bits(n: usize) -> usize {
    // Assuming that a byte contains 8 bits.
    n << 0x3
}

/// Get number of bits in value.
///
/// # Parameters
/// - `T` - given type.
///
/// # Returns
/// Number of bits in value.
#[inline(always)]
pub const fn bits_per_type<T>() -> usize {
    bytes_to_bits(size_of::<T>())
}

/// Set the specific bit of given value.
///
/// # Parameters
/// - `value` - given value to change.
/// - `pos`   - given bit position to set.
#[inline(always)]
pub fn set_bit<T>(value: &mut T, pos: usize)
where
    T: BitOrAssign + Shl<usize, Output = T> + From<u8>,
{
    let mask = T::from(1u8) << pos;
    *value |= mask;
}

/// Clear the specific bit of given value.
///
/// # Parameters
/// - `value` - given value to change.
/// - `pos`   - given bit position to set.
#[inline(always)]
pub fn clear_bit<T>(value: &mut T, pos: usize)
where
    T: BitAndAssign + Shl<usize, Output = T> + Not<Output = T> + From<u8>,
{
    let mask = !(T::from(1u8) << pos);
    *value &= mask;
}

/// Get the specific bit of given value.
///
/// # Parameters
/// - `value` - given value to test.
/// - `pos`   - given bit position to test.
#[inline(always)]
pub fn test_bit<T>(value: &T, pos: usize) -> bool
where
    T: BitAndAssign + Shl<usize, Output = T> + BitAnd<Output = T> + From<u8> + Copy + PartialOrd,
{
    let mask = T::from(1u8) << pos;
    (*value & mask) != T::from(0u8)
}

