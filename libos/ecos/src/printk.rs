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

//! Contains libOS log functions.

use crate::subsystem::graphics::{GraphicsSub, terminal::Terminal, Rgb, Color};
use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt;

/// Global text output foreground color.
pub static mut FOREGROUND_COLOR: Rgb = Color::White as Rgb;

/// Global text output background color.
pub static mut BACKGROUND_COLOR: Rgb = Color::Black as Rgb;

/// Set current color of text output to the screen.
///
/// # Parameters
/// - `fg` - given foreground color to set.
/// - `bg` - given background color to set.
pub fn set_color(fg: Rgb, bg: Rgb) {
    unsafe {
        FOREGROUND_COLOR = fg;
        BACKGROUND_COLOR = bg;
    }
}

/// This method should be implemented for Terminal in order to
/// create println!() like macro rules for kernel needs
impl fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            for c in s.chars() {
                self.putc(c, FOREGROUND_COLOR, BACKGROUND_COLOR);
            }
        }
        Ok(())
    }
}

lazy_static! {
    /// Global mutable Terminal object needed for printk!() like macro rules.
    pub static ref WRITER: Mutex<Terminal> = Mutex::new(Terminal::default());
}

/// Initialize libOS logs writer.
///
/// # Parameters:
/// - `gfx` - given graphics handeling object.
pub fn init(gfx: GraphicsSub) {
    WRITER.lock().init(gfx);
}

/// Formats and prints colored data.
#[macro_export]
macro_rules! cputk {
    ($fg:expr, $bg:expr, $($arg:tt)*) => (
        let (cur_fg, cur_bg) = (
            crate::printk::FOREGROUND_COLOR,
            crate::printk::BACKGROUND_COLOR
        );

        $crate::printk::set_color($fg, $bg);
        $crate::printk::_print(format_args!($($arg)*));
        $crate::printk::set_color(cur_fg, cur_bg);
    );
}

/// Formats and prints data.
#[macro_export]
macro_rules! putk {
    ($($arg:tt)*) => (
        $crate::printk::_print(format_args!($($arg)*))
    );
}

/// Prints string.
#[macro_export]
macro_rules! putsk {
    ($($arg:tt)*) => (
        $crate::printk::_print_str(format_args!($($arg)*))
    );
}

/// Formats and prints data with '\n' in the end.
#[macro_export]
macro_rules! printk {
    // Empty message.
    () => ($crate::print!("\n"));
    // Default case for any other arguments.
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

/// Prints string.
///
/// # Parameters
/// - `args` - given precompiled version of a string.
pub fn _print_str(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
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
/// [ OK   ] Operation completed successfully.
/// [ OK   ] Value is: 42
/// ```
#[macro_export]
macro_rules! pr_ok {
    ($($arg:tt)*) => (
        $crate::putsk!("[ ");
        unsafe {
            $crate::cputk!(
                crate::subsystem::graphics::Color::Yellow as u32,
                crate::printk::BACKGROUND_COLOR,
                "OK"
            );
        }
        $crate::putk!("  ] {}\n", format_args!($($arg)*))
    );
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
    ($($arg:tt)*) => (
        $crate::putsk!("[");
        unsafe {
            $crate::cputk!(
                crate::subsystem::graphics::Color::Yellow as u32,
                crate::printk::BACKGROUND_COLOR,
                "ERROR"
            );
        }
        $crate::putk!("] {}\n", format_args!($($arg)*))
    );
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
    ($($arg:tt)*) => (
        $crate::putsk!("[");
        unsafe {
            $crate::cputk!(
                crate::subsystem::graphics::Color::Yellow as u32,
                crate::printk::BACKGROUND_COLOR,
                "DEBUG"
            );
        }
        $crate::putsk!("] ");
        unsafe {
            $crate::cputk!(
                crate::subsystem::graphics::Color::Gray as u32,
                crate::printk::BACKGROUND_COLOR,
                "{}\n", format_args!($($arg)*)
            );
        }
    );
}

/// Log error messages that indicate libOS panic without halting CPU.
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
    ($($arg:tt)*) => (
        $crate::putsk!("[");
        unsafe {
            $crate::cputk!(
                crate::subsystem::graphics::Color::Yellow as u32,
                crate::printk::BACKGROUND_COLOR,
                "PANIC"
            );
        }
        $crate::putk!("] {}\n", format_args!($($arg)*))
    );
}

/// Log messages that provide detailed information useful for tests.
///
/// # Examples
///
/// ```rust
/// pr_test!("Entering function: {}", function_name);
/// ```
///
/// The output will be formatted as:
/// ```plaintext
/// [TEST ] Entering function: my_function
/// ```
#[macro_export]
macro_rules! pr_test {
    ($($arg:tt)*) => (
        $crate::putsk!("[");
        unsafe {
            $crate::cputk!(
                crate::subsystem::graphics::Color::Yellow as u32,
                crate::printk::BACKGROUND_COLOR,
                "TEST"
            );
        }
        $crate::putsk!(" ] ");
        unsafe {
            $crate::cputk!(
                crate::subsystem::graphics::Color::Gray as u32,
                crate::printk::BACKGROUND_COLOR,
                "{}\n", format_args!($($arg)*)
            );
        }
    );
}
