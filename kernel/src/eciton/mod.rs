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

//! Main kernel module.

pub mod multiboot;
pub mod printk;
mod graphics;
mod panic;

use crate::{
    eciton::{ multiboot::MultibootInfo, graphics::Graphics },
    printk
};

/// Initialize kernel.
///
/// # Parameters
/// - `boot_info` - given multiboot info structure.
pub fn init_kernel(boot_info: &MultibootInfo) {
    let gfx = Graphics::new(boot_info);
    printk::init(gfx);

    printk!("eciton exokernel v{}", "0.0.0");
}