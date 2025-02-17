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

//! EcOS log functions.

use crate::ecos::graphics::{Graphics, terminal};
use lazy_static::lazy_static;
use terminal::Terminal;
use spin::Mutex;
use core::fmt;

/// This method should be implemented for Terminal in order to
/// create println!() like macro rules for kernel needs
impl fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.putc(c, self.fg, self.bg);
        }
        Ok(())
    }
}

lazy_static! {
    /// Global mutable Terminal object needed for printk!() like macro rules.
    pub static ref WRITER: Mutex<Terminal> = Mutex::new(Terminal::default());
}

/// Initialize kernel logs writer.
///
/// # Parameters:
/// - `gfx` - given graphics handeling object.
pub fn init_printk(gfx: Graphics) {
    WRITER.lock().init(gfx);
}

/// Formats and prints data.
#[macro_export]
macro_rules! putk {
    ($($arg:tt)*) => (
        $crate::ecos::printk::_print(format_args!($($arg)*))
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
    WRITER.lock().write_fmt(args).unwrap();
}