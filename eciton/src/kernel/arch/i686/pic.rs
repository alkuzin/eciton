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

//! Provides definitions for Programmable Interrupt Controller (PIC).
//!
//! # Description
//! The Programmable Interrupt Controller (PIC) is a crucial component
//! in computer architecture, particularly in x86 systems, that manages
//! hardware interrupts. It allows the CPU to handle multiple interrupt
//! sources efficiently and provides a way to prioritize and route these
//! interrupts to the appropriate handlers.

use eciton_sdk::arch::i686::io::outb;

pub const MASTER_PIC_CMD: u16  = 0x20;
pub const MASTER_PIC_DATA: u16 = 0x21;
pub const SLAVE_PIC_CMD: u16   = 0xA0;
pub const SLAVE_PIC_DATA: u16  = 0xA1;

// PIC configurations:
pub const PIC_INIT_CMD: u8             = 0x11;
pub const MASTER_PIC_VECTOR_OFFSET: u8 = 0x20;
pub const SLAVE_PIC_VECTOR_OFFSET: u8  = 0x28;
pub const MASTER_PIC_CASCADE: u8       = 0x04;
pub const SLAVE_PIC_CASCADE: u8        = 0x02;
pub const PIC_8086_MODE: u8            = 0x01;
pub const ALL_INTERRUPTS_ENABLED: u8   = 0x00;
pub const END_OF_INTERRUPT: u8         = 0x20;

/// Initialize Programmable Interrupt Controller (PIC).
pub fn init() {
    unsafe {
        // Send the initialization command to both the master and slave PICs.
        outb(MASTER_PIC_CMD, PIC_INIT_CMD);
        outb(SLAVE_PIC_CMD,  PIC_INIT_CMD);

        // Set the vector offsets for the master and slave PICs.
        outb(MASTER_PIC_DATA, MASTER_PIC_VECTOR_OFFSET);
        outb(SLAVE_PIC_DATA,  SLAVE_PIC_VECTOR_OFFSET);

        // Configure the master and slave PICs for cascade operation.
        outb(MASTER_PIC_DATA, MASTER_PIC_CASCADE);
        outb(SLAVE_PIC_DATA,  SLAVE_PIC_CASCADE);

        // Set both PICs to operate in 8086/8088 mode.
        outb(MASTER_PIC_DATA, PIC_8086_MODE);
        outb(SLAVE_PIC_DATA,  PIC_8086_MODE);

        // Enable all interrupts on both PICs.
        outb(MASTER_PIC_DATA, ALL_INTERRUPTS_ENABLED);
        outb(SLAVE_PIC_DATA,  ALL_INTERRUPTS_ENABLED);
    }
}
