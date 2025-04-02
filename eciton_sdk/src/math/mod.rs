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

//! Contain standard mathematical functions.

pub use core::f64;

/// Calculate natural logarithm.
///
/// # Parameters
/// - `x` - given value.
///
/// # Returns
/// - Natural logarithm of `x`.
pub fn log(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN;
    }

    if x == 1.0 {
        return 0.0;
    }

    if x == f64::consts::E {
        return 1.0;
    }

    let mut i: i32      = 1;
    let mut result: f64 = 0.0;
    let mut term: f64   = (x - 1.0) / (x + 1.0);
    let square: f64     = term * term;

    while i <= 60000 {
        result += term / i as f64;
        term   *= square;
        i += 1;
        i += 1;
    }

    2.0 * result
}

/// Calculate the base-2 logarithm of a given value.
///
/// # Parameters
/// - `x` - given value.
///
/// # Returns
/// - Base-2 logarithm of `x`.
#[inline(always)]
pub fn log2(x: f64) -> f64 {
    log(x) / log(2.0)
}

/// Calculates the smallest integer value >= the given value.
///
/// # Parameters
/// - `x` - given value to ceil.
///
/// # Returns
/// - Ceil value of `x`.
pub fn ceil(x: f64) -> f64 {
    let int_part = x as i32;

    if (x - int_part as f64) < f64::EPSILON {
        x
    }
    else if x > 0.0 {
        (int_part + 1) as f64
    }
    else {
        int_part as f64
    }
}

/// Get the closest power of 2.
///
/// # Parameters
/// - `n` - given number.
///
/// # Returns
/// - The closest power of 2.
pub const fn roundup_pow_of_two(mut n: usize) -> usize {
    if n == 0 {
        return 1;
    }

    n -= 1;             // Handle the case where x is already a power of 2.
    n |= n >> 0x1;      // Set bits 1 and 2.
    n |= n >> 0x2;      // Set bits 1-4.
    n |= n >> 0x4;      // Set bits 1-8.
    n |= n >> 0x8;      // Set bits 1-16.
    n |= n >> 0x10;     // Set bits 1-32 (for 32-bit integers).

    n + 1
}
