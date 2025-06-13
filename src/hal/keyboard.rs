// SPDX-License-Identifier: GPL-3.0-or-later
// Date: 2025-06-13
// Author: Alexander Kuzin <alkuzindev@gmail.com>.

//! Keyboard driver architecture-independent declarations.

use crate::arch;

/// Keyboard key enumeration.
#[derive(Debug)]
#[repr(u8)]
pub enum Key {
    Esc,
    Backspace,
    Tab,
    Enter,
    LCtrl,
    LShift,
    Backslash,
    LAlt,
    Space,
    CapsLock,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    Unknown,
    Char(char),
}

/// Read keyboard key.
///
/// # Returns
/// - Pressed keyboard key info.
pub fn read_key() -> Key {
    #[cfg(target_arch = "x86")]
    arch::x86::drivers::keyboard::read_key()
}

/// Read line from keyboard till '\n' character.
///
/// # Parameters
/// - `buffer` - given buffer to store read characters.
pub fn get_line(buffer: &mut [char]) {
    #[cfg(target_arch = "x86")]
    arch::x86::drivers::keyboard::get_line(buffer);
}
