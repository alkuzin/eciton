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

//! LibOS entry point.

#![no_std]                      // Do not use the standard library.
#![no_main]                     // Do not use the standard main function.
#![allow(clippy::empty_loop)]   // Ignore empty loop.
#![allow(dead_code)]            // Allow unused values.

/// EcitonSDK crate.
extern crate sdk;

pub mod printk;
mod graphics;
mod api;

use crate::{graphics::{Framebuffer, Graphics}, api::exo};

/// LibOS entry point.
pub fn libos_main() -> ! {
    let mut fb = Framebuffer::default();
    exo::getfb(&mut fb);

    let gfx = Graphics::new(fb);
    printk::init(gfx);

    pr_ok!("Initialized EcOS.");

    // Halt libOS.
    loop {}
}