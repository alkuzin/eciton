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

//! EcOS - default libOS entry point.

#![no_std]                      // Do not use the standard library.
#![no_main]                     // Do not use the standard main function.
#![allow(clippy::empty_loop)]   // Ignore empty loop.
#![allow(dead_code)]            // Allow unused values.

mod ecos;

use exo::kernel::{graphics::Framebuffer, multiboot::MultibootInfo};
use ecos::{graphics, printk::init_printk};

static mut BOOT_INFO: Option<&MultibootInfo> = None;

/// CRUTCH: should be replaced with normal syscall!
pub fn set_multiboot_info(boot_info: &'static MultibootInfo) {
    unsafe { BOOT_INFO = Some(boot_info); }
}

pub fn libos_init() {
    // TODO: syscall to get framebuffer
    // exo::libos::get_framebuffer(&mut fb);
    let fb  = Framebuffer::new(unsafe {BOOT_INFO.unwrap()});
    let gfx = graphics::Graphics::new(fb);
    init_printk(gfx);

    printk!("initialized EcOS");
}