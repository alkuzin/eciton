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
mod vesa;
mod gfx;

use vesa::Vesa;
use multiboot::MultibootInfo;
use gfx::{Color, Rgb};


pub fn setup_kernel(boot_info: &MultibootInfo)
{
    let vesa = Vesa::new(boot_info);

    for i in 0..300 {
        vesa.put_pixel(i, i, Color::White as u32);
        vesa.put_pixel(300, i, Color::Red as u32);
    }

    vesa.draw_char('A', 500, 500, Color::Green as Rgb);
}