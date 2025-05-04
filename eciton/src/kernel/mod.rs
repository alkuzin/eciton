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
pub mod tests;
mod syscall;
mod drivers;
mod memory;
mod debug;
mod arch;

use crate::{kernel::{
    arch::i686::{gdt, idt}, drivers::uart::Uart, multiboot::MultibootInfo
}, ecos, pr_ok};
use eciton_sdk::context::Context;
use tests::*;

/// Initialize kernel.
///
/// # Parameters
/// - `boot_info` - given multiboot info structure.
pub fn init(_boot_info: &MultibootInfo) {
    if Uart::init().is_ok() {
        pr_ok!("Initialized UART driver.");
    }

    gdt::init();
    pr_ok!("Initialized Global Descriptor Table.");

    idt::init();
    pr_ok!("Initialized Interrupt Descriptor Table.");

    memory::init();
    pr_ok!("Initialized kernel memory manager.");

    syscall::init();
    pr_ok!("Initialized system call handler.");

    // Run tests for these modules.
    exotest_run_modules!(memory, syscall);

    pr_ok!("Running default libOS.");
    ecos::libos_main(Context::default());
}