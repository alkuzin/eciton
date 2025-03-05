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

//! Provides definitions for Interrupt Descriptor Table (IDT).
//!
//! # Description
//! The Interrupt Descriptor Table (IDT) is a crucial data structure in x86
//! architecture that is used to manage interrupts and exceptions.
//! It allows the CPU to respond to various events, such as hardware
//! interrupts, software interrupts, and exceptions.

use eciton_sdk::arch::i686::{idt::{Entry, Pointer}, gdt::Segment};
use super::{pic, irq};
use core::ffi::c_void;

/// Number of IDT entries.
pub const IDT_ENTRIES: usize = 256;

/// Tells the CPU which segment to use when executing the interrupt handler.
const GATE_SELECTOR: u16 = Segment::KernelCode as u16;

/// Descriptor used to define an interrupt handler.
const INTERRUPT_GATE: u8 = 0x8E;

/// Empty entry.
const NULL_ENTRY: Entry = Entry {
    offset_low:  0,
    selector:    0,
    reserved:    0,
    flags:       0,
    offset_high: 0,
};

/// Interrupt Descriptor Table.
static mut IDT: [Entry;IDT_ENTRIES] = [NULL_ENTRY;IDT_ENTRIES];

/// Interrupt Descriptor Table pointer.
static mut IDT_PTR: Pointer = Pointer { size: 0, offset: 0 };

/// Initialize Interrupt Descriptor Table.
pub fn init() {
    pic::init();
    set_gates();
    set_pointer();

    unsafe {
        // Update IDT.
        idt_flush((&raw const IDT_PTR as *const _) as u32);
    }
}

/// Set the IDT gate.
///
/// # Parameters
/// - `num`    - given IDT number.
/// - `offset` - given entry point of the interrupt function.
#[inline(always)]
pub fn set_gate(num: usize, offset: u32) {
    unsafe {
        IDT[num] = Entry::new(offset, GATE_SELECTOR, INTERRUPT_GATE);
    }
}

/// Convert function pointer to u32.
///
/// # Parameters
/// - `ptr` - given function pointer to convert.
///
/// # Returns
/// Function pointer address as u32.
#[inline(always)]
fn fn_ptr_to_u32(ptr: unsafe extern "C" fn()) -> u32 {
    (ptr as *const c_void) as u32
}

/// Set IDT gates.
fn set_gates() {
    // Set gates for ISR functions for hardware interrupts 0-31.
    set_gate(0,  fn_ptr_to_u32(irq::isr0));
    set_gate(1,  fn_ptr_to_u32(irq::isr1));
    set_gate(2,  fn_ptr_to_u32(irq::isr2));
    set_gate(3,  fn_ptr_to_u32(irq::isr3));
    set_gate(4,  fn_ptr_to_u32(irq::isr4));
    set_gate(5,  fn_ptr_to_u32(irq::isr5));
    set_gate(6,  fn_ptr_to_u32(irq::isr6));
    set_gate(7,  fn_ptr_to_u32(irq::isr7));
    set_gate(8,  fn_ptr_to_u32(irq::isr8));
    set_gate(9,  fn_ptr_to_u32(irq::isr9));
    set_gate(10, fn_ptr_to_u32(irq::isr10));
    set_gate(11, fn_ptr_to_u32(irq::isr11));
    set_gate(12, fn_ptr_to_u32(irq::isr12));
    set_gate(13, fn_ptr_to_u32(irq::isr13));
    set_gate(14, fn_ptr_to_u32(irq::isr14));
    set_gate(15, fn_ptr_to_u32(irq::isr15));
    set_gate(16, fn_ptr_to_u32(irq::isr16));
    set_gate(17, fn_ptr_to_u32(irq::isr17));
    set_gate(18, fn_ptr_to_u32(irq::isr18));
    set_gate(19, fn_ptr_to_u32(irq::isr19));
    set_gate(20, fn_ptr_to_u32(irq::isr20));
    set_gate(21, fn_ptr_to_u32(irq::isr21));
    set_gate(22, fn_ptr_to_u32(irq::isr22));
    set_gate(23, fn_ptr_to_u32(irq::isr23));
    set_gate(24, fn_ptr_to_u32(irq::isr24));
    set_gate(25, fn_ptr_to_u32(irq::isr25));
    set_gate(26, fn_ptr_to_u32(irq::isr26));
    set_gate(27, fn_ptr_to_u32(irq::isr27));
    set_gate(28, fn_ptr_to_u32(irq::isr28));
    set_gate(29, fn_ptr_to_u32(irq::isr29));
    set_gate(30, fn_ptr_to_u32(irq::isr30));
    set_gate(31, fn_ptr_to_u32(irq::isr31));

    // Set gates for IRQ functions for system calls.
    set_gate(32, fn_ptr_to_u32(irq::irq0));
    set_gate(33, fn_ptr_to_u32(irq::irq1));
    set_gate(34, fn_ptr_to_u32(irq::irq2));
    set_gate(35, fn_ptr_to_u32(irq::irq3));
    set_gate(36, fn_ptr_to_u32(irq::irq4));
    set_gate(37, fn_ptr_to_u32(irq::irq5));
    set_gate(38, fn_ptr_to_u32(irq::irq6));
    set_gate(39, fn_ptr_to_u32(irq::irq7));
    set_gate(40, fn_ptr_to_u32(irq::irq8));
    set_gate(41, fn_ptr_to_u32(irq::irq9));
    set_gate(42, fn_ptr_to_u32(irq::irq10));
    set_gate(43, fn_ptr_to_u32(irq::irq11));
    set_gate(44, fn_ptr_to_u32(irq::irq12));
    set_gate(45, fn_ptr_to_u32(irq::irq13));
    set_gate(46, fn_ptr_to_u32(irq::irq14));
    set_gate(47, fn_ptr_to_u32(irq::irq15));

    // Eciton kernel syscall (0x66).
    set_gate(102, fn_ptr_to_u32(irq::isr102));

    // Set gates for ISR functions for hardware interrupts 0-15.
    set_gate(128, fn_ptr_to_u32(irq::isr128));
    set_gate(177, fn_ptr_to_u32(irq::isr177));
}

// Set IDT pointer.
fn set_pointer() {
    unsafe {
        IDT_PTR.size   = (size_of::<Entry>() * IDT_ENTRIES - 1) as u16;
        IDT_PTR.offset = (&raw const IDT as *const _) as u32;
    }
}

unsafe extern "C" {
    /// Flush out the old IDT and install the new changes.
    ///
    /// # Parameters
    /// - `ptr` - given new IDT pointer to update.
    unsafe fn idt_flush(ptr: u32);
}