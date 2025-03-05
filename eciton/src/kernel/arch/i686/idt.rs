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

use crate::kernel::arch::i686::{gdt, pic, irq};
use core::ffi::c_void;

/// IDT gate descriptor structure in 32-bit mode.
#[repr(C, packed)]
struct Entry {
    /// Entry point of the ISR (lower bits).
    pub offset_low: u16,
    /// Point to a valid code segment in GDT.
    pub selector: u16,
    /// Unused.
    pub reserved: u8,
    /// Gate type & other control bits.
    pub flags: u8,
    /// Entry point of the ISR (higher bits).
    pub offset_high: u16,
}

/// IDT pointer.
#[repr(C, packed)]
struct Pointer {
    /// Size of IDT.
    pub size: u16,
    /// The linear address of the IDT.
    pub offset: u32,
}

/// Number of IDT entries.
pub const IDT_ENTRIES: usize = 256;

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

/// Set the IDT gate.
///
/// # Parameters
/// - `num`      - given IDT number.
/// - `offset`   - given entry point of the ISR.
/// - `selector` - given point to a valid code segment in GDT.
/// - `flags`    - given gate type & other control bits.
fn set_gate(num: usize, offset: u32, selector: u16, flags: u8) {
    unsafe {
        IDT[num].offset_low  = (offset & 0xFFFF) as u16;
        IDT[num].selector    = selector;
        IDT[num].reserved    = 0;
        IDT[num].flags       = flags | 0x60;
        IDT[num].offset_high = ((offset >> 0x10) & 0xFFFF) as u16;
    }
}

/// Set the IDT gate.
///
/// # Parameters
/// - `num`    - given IDT number.
/// - `offset` - given entry point of the interrupt function.
#[inline(always)]
pub fn set_int_function(num: usize, offset: u32) {
    set_gate(num, offset, gdt::Segment::KernelCode as u16, INTERRUPT_GATE);
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
    set_int_function(0,  fn_ptr_to_u32(irq::isr0));
    set_int_function(1,  fn_ptr_to_u32(irq::isr1));
    set_int_function(2,  fn_ptr_to_u32(irq::isr2));
    set_int_function(3,  fn_ptr_to_u32(irq::isr3));
    set_int_function(4,  fn_ptr_to_u32(irq::isr4));
    set_int_function(5,  fn_ptr_to_u32(irq::isr5));
    set_int_function(6,  fn_ptr_to_u32(irq::isr6));
    set_int_function(7,  fn_ptr_to_u32(irq::isr7));
    set_int_function(8,  fn_ptr_to_u32(irq::isr8));
    set_int_function(9,  fn_ptr_to_u32(irq::isr9));
    set_int_function(10, fn_ptr_to_u32(irq::isr10));
    set_int_function(11, fn_ptr_to_u32(irq::isr11));
    set_int_function(12, fn_ptr_to_u32(irq::isr12));
    set_int_function(13, fn_ptr_to_u32(irq::isr13));
    set_int_function(14, fn_ptr_to_u32(irq::isr14));
    set_int_function(15, fn_ptr_to_u32(irq::isr15));
    set_int_function(16, fn_ptr_to_u32(irq::isr16));
    set_int_function(17, fn_ptr_to_u32(irq::isr17));
    set_int_function(18, fn_ptr_to_u32(irq::isr18));
    set_int_function(19, fn_ptr_to_u32(irq::isr19));
    set_int_function(20, fn_ptr_to_u32(irq::isr20));
    set_int_function(21, fn_ptr_to_u32(irq::isr21));
    set_int_function(22, fn_ptr_to_u32(irq::isr22));
    set_int_function(23, fn_ptr_to_u32(irq::isr23));
    set_int_function(24, fn_ptr_to_u32(irq::isr24));
    set_int_function(25, fn_ptr_to_u32(irq::isr25));
    set_int_function(26, fn_ptr_to_u32(irq::isr26));
    set_int_function(27, fn_ptr_to_u32(irq::isr27));
    set_int_function(28, fn_ptr_to_u32(irq::isr28));
    set_int_function(29, fn_ptr_to_u32(irq::isr29));
    set_int_function(30, fn_ptr_to_u32(irq::isr30));
    set_int_function(31, fn_ptr_to_u32(irq::isr31));

    // Set gates for IRQ functions for system calls.
    set_int_function(32, fn_ptr_to_u32(irq::irq0));
    set_int_function(33, fn_ptr_to_u32(irq::irq1));
    set_int_function(34, fn_ptr_to_u32(irq::irq2));
    set_int_function(35, fn_ptr_to_u32(irq::irq3));
    set_int_function(36, fn_ptr_to_u32(irq::irq4));
    set_int_function(37, fn_ptr_to_u32(irq::irq5));
    set_int_function(38, fn_ptr_to_u32(irq::irq6));
    set_int_function(39, fn_ptr_to_u32(irq::irq7));
    set_int_function(40, fn_ptr_to_u32(irq::irq8));
    set_int_function(41, fn_ptr_to_u32(irq::irq9));
    set_int_function(42, fn_ptr_to_u32(irq::irq10));
    set_int_function(43, fn_ptr_to_u32(irq::irq11));
    set_int_function(44, fn_ptr_to_u32(irq::irq12));
    set_int_function(45, fn_ptr_to_u32(irq::irq13));
    set_int_function(46, fn_ptr_to_u32(irq::irq14));
    set_int_function(47, fn_ptr_to_u32(irq::irq15));

    // Eciton kernel syscall (0x66).
    set_int_function(102, fn_ptr_to_u32(irq::isr102));

    // Set gates for ISR functions for hardware interrupts 0-15.
    set_int_function(128, fn_ptr_to_u32(irq::isr128));
    set_int_function(177, fn_ptr_to_u32(irq::isr177));
}


unsafe extern "C" {
    /// Flush out the old IDT and install the new changes.
    ///
    /// # Parameters
    /// - `ptr` - given new IDT pointer to update.
    unsafe fn idt_flush(ptr: u32);
}

/// Initialize Interrupt Descriptor Table.
pub fn init() {
    pic::init();
    set_gates();

    unsafe {
        // Set pointer structure to IDT.
        IDT_PTR.size   = (size_of::<Entry>() * IDT_ENTRIES - 1) as u16;
        IDT_PTR.offset = (&raw const IDT as *const _) as u32;

        // Update IDT.
        idt_flush((&raw const IDT_PTR as *const _) as u32);
    }
}