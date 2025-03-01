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
    let message  = info.message().as_str().unwrap_or("Unknown panic");
    let location = info.location().unwrap();

    printk!(
        "[KERNEL PANIC] Panic occured in file: '{}' on line: {} at column: {}",
        location.file(),
        location.line(),
        location.column(),
    );

    // The "EXCEPTION" message is used to signal panic! to not
    // print panic messages at all, because this macro can't
    // print formatted panic message in case of exceptions.
    // For displaying detailed panic messages in case of exception,
    // panic! is used along with printk! macro.
    if !message.starts_with("EXCEPTION") {
        printk!("Message: {}", message);
    }

    // Halt kernel.
    loop {}
}
