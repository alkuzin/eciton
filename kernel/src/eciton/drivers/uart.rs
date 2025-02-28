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

//! Contains UART (Universal Asynchronous Receiver-Transmitter) driver.

use crate::eciton::arch::i686::io;

/// Base address for COM1.
const UART_BASE: u16 = 0x3f8;

pub struct Uart {}

impl Uart {
    /// Initialize UART driver.
    ///
    /// # Returns
    /// - `Ok()`  - in case of success.
    /// - `Err()` - otherwise.
    pub fn init() -> Result<(),()> {
        unsafe {
            // Disable all interrupts.
            io::outb(UART_BASE + 1, 0x00);
            // Enable DLAB (set baud rate divisor).
            io::outb(UART_BASE + 3, 0x80);
            // Set divisor to 3 (lo byte) 38400 baud.
            io::outb(UART_BASE, 0x03);
            // Set divisor to 3 (hi byte) 38400 baud.
            io::outb(UART_BASE + 1, 0x00);
            // 8 bits, no parity, one stop bit.
            io::outb(UART_BASE + 3, 0x03);
            // Enable FIFO, clear them, with 14-byte threshold.
            io::outb(UART_BASE + 2, 0xC7);
            // IRQs enabled, RTS/DSR set.
            io::outb(UART_BASE + 4, 0x0B);
            // Set in loopback mode, test the serial chip.
            io::outb(UART_BASE + 4, 0x1E);
            // Test serial chip (send byte 0xAE and check
            // if serial returns same byte).
            io::outb(UART_BASE, 0xAE);

            // Check if serial is faulty (i.e: not same byte as sent).
            if io::inb(UART_BASE) != 0xAE {
                return Err(());
            }

            // Set it in normal operation mode.
            io::outb(UART_BASE + 4, 0x0F);
        }

        Ok(())
    }

    /// Write character to serial port.
    ///
    /// # Parameters
    /// - `c` - given character to write.
    pub fn write(c: char) {
        while !Self::is_transmit_empty() {
            continue;
        }

        unsafe {
            io::outb(UART_BASE, c as u8);
        }
    }

    /// Checks if the transmit buffer is empty.
    ///
    /// # Returns
    /// - `true`  - if the transmit buffer is empty.
    /// - `false` - otherwise.
    fn is_transmit_empty() -> bool {
        unsafe {
            // Read the Line Status Register and check
            // the transmit empty bit (bit 5).
            (io::inb(UART_BASE + 5) & 0x20) != 0
        }
    }

}