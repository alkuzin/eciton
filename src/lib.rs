// SPDX-License-Identifier: GPL-3.0-or-later
// Date: 2025-06-13
// Author: Alexander Kuzin <alkuzindev@gmail.com>.

//! Kernel entry point.

// Do not use the standard library.
#![no_std]
// Do not use the standard main function.
#![no_main]
// Ignore empty loop.
#![allow(clippy::empty_loop)]
// Allow unused values.
#![allow(dead_code)]

mod arch;
mod config;
mod drivers;
mod hal;
mod kernel;
mod log;
mod multiboot;

use multiboot::{MULTIBOOT_BOOTLOADER_MAGIC, MultibootInfo};

/// Kernel entry point.
///
/// # Parameters
/// - `magic`     - given multiboot magic number.
/// - `boot_info` - given multiboot info structure.
#[unsafe(no_mangle)]
extern "C" fn kmain(magic: u32, boot_info: &MultibootInfo) -> ! {
    // Check that multiboot magic number is correct.
    assert_eq!(magic, MULTIBOOT_BOOTLOADER_MAGIC);

    // Initialize the kernel.
    kernel::init(boot_info);

    // Halt the kernel.
    loop {}
}

use core::panic::PanicInfo;

/// Custom kernel panic handler.
///
/// # Parameters
/// - `info` - given panic information struct.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
