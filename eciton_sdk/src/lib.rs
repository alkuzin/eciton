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

//! EcitonSDK main module.

#![no_std]                      // Do not use the standard library.
#![no_main]                     // Do not use the standard main function.
#![allow(clippy::empty_loop)]   // Ignore empty loop.
#![allow(dead_code)]            // Allow unused values.

pub mod collections;
pub mod context;
pub mod bitops;
pub mod arch;
pub mod math;
pub mod vbe;

/// Trait for memory units conversion.
pub trait MemoryUnit {
    /// Convert type representing bytes to KB.
    fn kb(&self) -> usize;

    /// Convert type representing bytes to MB.
    fn mb(&self) -> usize;

    /// Convert type representing bytes to GB.
    fn gb(&self) -> usize;
}

/// Implementation of memory units conversion for usize.
impl MemoryUnit for usize {
    /// Convert type representing bytes to KB.
    ///
    /// # Returns
    /// - The number of KB from given number of bytes.
    #[inline(always)]
    fn kb(&self) -> usize {
        self << 10
    }

    /// Convert type representing bytes to MB.
    ///
    /// # Returns
    /// - The number of MB from given number of bytes.
    #[inline(always)]
    fn mb(&self) -> usize {
        self << 20
    }

    /// Convert type representing bytes to GB.
    ///
    /// # Returns
    /// - The number of GB from given number of bytes.
    #[inline(always)]
    fn gb(&self) -> usize {
        self << 30
    }
}