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

//! Main kernel module. Responsible for initializing kernel components.

pub mod multiboot;
pub mod printk;
pub mod panic;
mod drivers;
mod arch;

use crate::{
    eciton::{
        multiboot::MultibootInfo,
        drivers::uart::Uart,
        arch::i686::gdt,
        arch::i686::idt,
    },
    printk
};

/// Initialize kernel.
///
/// # Parameters
/// - `boot_info` - given multiboot info structure.
pub fn init_kernel(_boot_info: &'static MultibootInfo) {
    let _ = Uart::init();
    printk!("[  OK  ]: Initialized UART driver");

    gdt::init();
    printk!("[  OK  ]: Initialized Global Descriptor Table");

    idt::init();
    printk!("[  OK  ]: Initialized Interrupt Descriptor Table");
    // TODO: add pr_info() pr_error() ...
    // TODO: add kernel debug functions (read/write register, kdump)
    // TODO: add registers output during kernel panic
}