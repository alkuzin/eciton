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

//! Declares kernel memory layout.

// Declared in linker.ld file.
unsafe extern "C" {
    pub unsafe static kernel_phys_begin: u32;
    pub unsafe static kernel_phys_end:   u32;
    pub unsafe static kernel_virt_begin: u32;
    pub unsafe static kernel_virt_end:   u32;
    pub unsafe static base_address:      u32;
}

/// Get kernel begin.
///
/// # Returns
/// Kernel begin physical address.
#[inline(always)]
pub fn kernel_begin_paddr() -> u32 {
    unsafe {
        (&kernel_phys_begin as *const _) as u32
    }
}

/// Get kernel end.
///
/// # Returns
/// Kernel end physical address.
#[inline(always)]
pub fn kernel_end_paddr() -> u32 {
    unsafe {
        (&kernel_phys_end as *const _) as u32
    }
}

/// Get kernel begin.
///
/// # Returns
/// Kernel begin virtual address.
#[inline(always)]
pub fn kernel_begin_vaddr() -> u32 {
    unsafe {
        (&kernel_virt_begin as *const _) as u32
    }
}

/// Get kernel end.
///
/// # Returns
/// Kernel end virtual address.
#[inline(always)]
pub fn kernel_end_vaddr() -> u32 {
    unsafe {
        (&kernel_virt_end as *const _) as u32
    }
}

/// Get kernel base address.
///
/// # Returns
/// Kernel virtual base address.
#[inline(always)]
pub fn base_vaddr() -> u32 {
    unsafe {
        (&base_address as *const _) as u32
    }
}

/// Get kernel size.
///
/// # Returns
/// Kernel size in bytes.
#[inline(always)]
pub fn kernel_size() -> u32 {
    kernel_end_paddr() - kernel_begin_paddr()
}

/// Physical address of memory begin.
pub const MEM_START_PADDR: u32 = 0x00000000;

/// Size of kernel stack (64 KB).
pub const STACK_SIZE: usize = 65336;