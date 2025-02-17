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

pub mod printk;
mod panic;
mod arch;
mod uart;

use exo::kernel::multiboot::MultibootInfo;
use crate::{eciton::uart::Uart, printk};

/// Initialize kernel.
///
/// # Parameters
/// - `boot_info` - given multiboot info structure.
pub fn init_kernel(boot_info: &'static MultibootInfo) {
    // TODO: register & init libOS.
    let _ = Uart::init();
    printk!("[  OK  ]: Initialized UART driver");

    printk!("[  OK  ]: Set multiboot info for EcOS libOS");
    ecos::set_multiboot_info(boot_info);

    printk!("[  OK  ]: Initializing libOS");
    ecos::libos_init()
}