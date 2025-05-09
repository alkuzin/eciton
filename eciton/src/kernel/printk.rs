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

//! Contains kernel log functions.

use super::drivers::uart::Uart;
use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt;

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            Uart::write(c);
        }
        Ok(())
    }
}

lazy_static! {
    pub static ref SERIAL: Mutex<Uart> = Mutex::new(Uart {});
}

/// Formats and prints data.
#[macro_export]
macro_rules! putk {
    ($($arg:tt)*) => (
        $crate::kernel::printk::_print(format_args!($($arg)*))
    );
}

/// Formats and prints data with '\n' in the end.
#[macro_export]
macro_rules! printk {
    // Empty message.
    () => ($crate::putk!("\n"));
    // Default case for any other arguments.
    ($($arg:tt)*) => ($crate::putk!("{}\n", format_args!($($arg)*)));
}

/// Prints format string and it's arguments.
///
/// # Parameters
/// - `args` - given precompiled version of a format string and it`s arguments.
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL.lock().write_fmt(args).unwrap();
}

/// Log informational messages that indicate successful operations or states.
///
/// # Examples
///
/// ```rust
/// pr_ok!("Operation completed successfully.");
/// pr_ok!("Value is: {}", value);
/// ```
///
/// The output will be formatted as:
/// ```plaintext
/// [  OK  ] Operation completed successfully.
/// [  OK  ] Value is: 42
/// ```
#[macro_export]
macro_rules! pr_ok {
    ($($arg:tt)*) => ($crate::putk!("[ OK  ] {}\n", format_args!($($arg)*)));
}

/// Log error messages that indicate a failure or an unexpected condition.
///
/// # Examples
///
/// ```rust
/// pr_err!("An error occurred: {}", error_message);
/// ```
///
/// The output will be formatted as:
/// ```plaintext
/// [ERROR] An error occurred: File not found
/// ```
#[macro_export]
macro_rules! pr_err {
    ($($arg:tt)*) => ($crate::putk!("[ERROR] {}\n", format_args!($($arg)*)));
}

/// Log messages that provide detailed information useful for debugging purposes.
///
/// # Examples
///
/// ```rust
/// pr_debug!("Entering function: {}", function_name);
/// ```
///
/// The output will be formatted as:
/// ```plaintext
/// [DEBUG] Entering function: my_function
/// ```
#[macro_export]
macro_rules! pr_debug {
    ($($arg:tt)*) => ($crate::putk!("[DEBUG] {}\n", format_args!($($arg)*)));
}

/// Log error messages that indicate kernel panic without halting CPU.
///
/// # Examples
///
/// ```rust
/// pr_panic!("Kernel panic in function: {}", function_name);
/// ```
///
/// The output will be formatted as:
/// ```plaintext
/// [PANIC] Kernel panic in function: my_function
/// ```
#[macro_export]
macro_rules! pr_panic {
    ($($arg:tt)*) => ($crate::putk!("[PANIC] {}\n", format_args!($($arg)*)));
}