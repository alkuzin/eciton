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
pub mod tty;

use multiboot::MultibootInfo;
use vesa::Vesa;

use crate::printk;
struct Context<'a> {
    pub boot_info: &'a MultibootInfo,
}

impl<'a> Context<'a> {
    pub fn new(boot_info: &'a MultibootInfo) -> Context<'a> {
        Context {
            boot_info,
        }
    }
}

// TODO: make Cluster as trait in order to register it
struct Cluster<'a> {
    context: Context<'a>,
    vesa:    Vesa,
}

impl<'a> Cluster<'a> {
    pub fn new(boot_info: &'a MultibootInfo) -> Cluster<'a> {
        let context = Context::new(boot_info);

        Cluster {
            context,
            vesa: Default::default(),
        }
    }

    pub fn init(&mut self) {
        self.vesa = Vesa::new(self.context.boot_info);
        tty::WRITER.lock().set(self.vesa);
    }
}

pub fn setup_kernel(boot_info: &MultibootInfo) {
    let mut cluster = Cluster::new(boot_info);
    cluster.init();

    printk!("eciton exokernel v{}", "0.0.0");
}