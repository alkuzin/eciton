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
mod debug;
mod arch;

use crate::{
    eciton::{
        arch::i686::{gdt, idt},
        drivers::uart::Uart,
        multiboot::MultibootInfo
    }, pr_ok
};

/// Initialize kernel.
///
/// # Parameters
/// - `boot_info` - given multiboot info structure.
pub fn init_kernel(_boot_info: &'static MultibootInfo) {
    if Uart::init().is_ok() {
        pr_ok!("Initialized UART driver.");
    }

    gdt::init();
    pr_ok!("Initialized Global Descriptor Table.");

    idt::init();
    pr_ok!("Initialized Interrupt Descriptor Table.");
}