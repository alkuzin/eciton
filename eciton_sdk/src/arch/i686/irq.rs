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

//! Provides definitions for Interrupt Requests (IRQ).

/// Interrupt register state struct.
#[derive(Debug, Default)]
#[repr(C, packed)]
pub struct IntRegisterState {
    /// Control register 2.
    pub cr2: u32,
    /// Data segment.
    pub ds: u32,
    /// Sestination index.
    pub edi: u32,
    /// Source index.
    pub esi: u32,
    /// Base pointer.
    pub ebp: u32,
    /// Stack pointer.
    pub esp: u32,
    /// Base register.
    pub ebx: u32,
    /// Data register.
    pub edx: u32,
    /// Counter register.
    pub ecx: u32,
    /// Accumulator register.
    pub eax: u32,
    /// Interrupt number.
    pub int_no: u32,
    /// Error code.
    pub err_code: u32,
    /// Instruction pointer.
    pub eip: u32,
    /// Code segment.
    pub cs: u32,
    /// Flags register.
    pub eflags: u32,
    /// User stack pointer.
    pub useresp: u32,
    /// Stack segment.
    pub ss: u32,
}

/// Interrupt requests handler function alias.
pub type InterruptHandler = fn (&IntRegisterState);

/// Number of exception messages.
pub const EXCEPTION_SIZE: usize = 32;

/// List of CPU exception messages.
pub const EXCEPTION_MESSSAGES: [&str;EXCEPTION_SIZE] = [
    "Division by zero",
    "Debug",
    "Non maskable interrupt",
    "Breakpoint",
    "Into detected overflow",
    "Out of bounds",
    "Invalid opcode",
    "No coprocessor",
    "Double fault",
    "Coprocessor segment overrun",
    "Bad TSS",
    "Segment not present",
    "Stack fault",
    "General protection fault",
    "Page fault",
    "Unknown interrupt",
    "Coprocessor fault",
    "Alignment fault",
    "Machine check",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved"
];

/// Interrupt requests number type.
pub type Irq = usize;