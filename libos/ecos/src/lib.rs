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
extern crate eciton_sdk;

pub mod subsystem;
pub mod printk;
mod api;

use crate::{api::LibOSCore, subsystem::{SubsystemsArray, graphics::Graphics}};
use eciton_sdk::context::Context;

/// LibOS entry point.
///
/// # Parameters
/// - `context` - given exokernel context structure.
pub fn libos_main(context: Context) -> ! {
    // Prepare subsystems before initialization.
    let mut gfx = Graphics::default();
    let subsystems: SubsystemsArray = [&mut gfx];

    // Initialize libOS core module.
    let mut libos_core = LibOSCore::new(context, subsystems);
    libos_core.init().unwrap();

    pr_ok!("Initialized EcOS.");

    // Halt libOS.
    loop {}
}