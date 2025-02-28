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

//! Kernel panic function.

use core::panic::PanicInfo;
use crate::printk;

/// Custom kernel panic handler.
///
/// # Parameters
/// - `info` - given panic information struct.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let message  = info.message().as_str().unwrap();
    let location = info.location().unwrap();

    printk!(
        "[panic]: file: {} line: {} column: {}\nmessage: \"{}\"\n",
        location.file(),
        location.line(),
        location.column(),
        message
    );

    // Halt kernel
    loop {}
}