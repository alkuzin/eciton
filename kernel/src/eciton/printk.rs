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

use crate::eciton::drivers::uart::Uart;

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
        $crate::eciton::printk::_print(format_args!($($arg)*))
    );
}

/// Formats and prints data with '\n' in the end.
#[macro_export]
macro_rules! printk {
    () => ($crate::putk!("\n"));
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