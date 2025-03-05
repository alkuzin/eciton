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

use crate::{
    kernel::arch::i686::{idt, pic, system::sti},
    pr_panic
};

pub use eciton_sdk::arch::i686::{
    io::outb,
    irq::{
        IntRegisterState, InterruptHandler, Irq,
        EXCEPTION_SIZE, EXCEPTION_MESSSAGES
    },
};

use core::ptr;

/// Null interrupt handler (used as placeholder for ROUTINES below).
fn null_handler(_ : &IntRegisterState) {
    // Do nothing.
}

/// Routines table type.
type Routines = [InterruptHandler;idt::IDT_ENTRIES];

/// Handlers that are designed to respond to hardware interrupts.
static mut ROUTINES: Routines = [null_handler;idt::IDT_ENTRIES];

/// Install handler for IRQ.
///
/// # Parameters
/// - `irq`     - given IRQ number.
/// - `handler` - given pointer to IRQ handler function.
pub fn request(irq: Irq, handler: InterruptHandler) {
    unsafe {
        ROUTINES[irq] = handler;
    }
    sti();
}

/// Uninstall handler for IRQ.
///
/// # Parameters
/// - `irq` - given IRQ number.
pub fn free(irq: Irq) {
    unsafe {
        ROUTINES[irq] = null_handler;
    }
}

/// IRQ handler function.
///
/// # Parameters
/// `regs` - given pointer to interrupt register state.
#[unsafe(no_mangle)]
pub extern "C" fn irq_handler(regs: &IntRegisterState) {
    // IRQ handler processes the interrupt by calling the appropriate
    // handler function based on the interrupt number.
    let handler = unsafe {
        ROUTINES[(regs.int_no - EXCEPTION_SIZE as u32) as usize]
    };

    // Handle interrupt if handler exists.
    let null_handler = null_handler as for<'a> fn(&'a IntRegisterState);

    if !ptr::fn_addr_eq(handler, null_handler) {
        handler(regs);
    }

    // This tells the slave PIC that interrupt handling was finished.
    if regs.int_no >= 40 {
        unsafe {
            outb(pic::SLAVE_PIC_CMD, pic::END_OF_INTERRUPT);
        }
    }

    // This tells the master PIC that interrupt handling was finished.
    unsafe {
        outb(pic::MASTER_PIC_CMD, pic::END_OF_INTERRUPT);
    }
}

/// ISR handler function.
///
/// # Parameters
/// - `regs` - given pointer to interrupt register state.
#[unsafe(no_mangle)]
pub extern "C" fn isr_handler(regs: &IntRegisterState) {
    // Handle exceptions.
    if regs.int_no < EXCEPTION_SIZE as u32 {
        let message = EXCEPTION_MESSSAGES[regs.int_no as usize];
        pr_panic!("Exception occured: '{}'", message);
        panic!("EXCEPTION");
    }

    // Handle interrupt if handler exists.
    let null_handler = null_handler as for<'a> fn(&'a IntRegisterState);
    let handler = unsafe { ROUTINES[regs.int_no as usize] };

    if !ptr::fn_addr_eq(handler, null_handler) {
        handler(regs);
    }
}

unsafe extern "C" {
    /// ISR functions for hardware interrupts 0-31.
    pub unsafe fn isr0();
    pub unsafe fn isr1();
    pub unsafe fn isr2();
    pub unsafe fn isr3();
    pub unsafe fn isr4();
    pub unsafe fn isr5();
    pub unsafe fn isr6();
    pub unsafe fn isr7();
    pub unsafe fn isr8();
    pub unsafe fn isr9();
    pub unsafe fn isr10();
    pub unsafe fn isr11();
    pub unsafe fn isr12();
    pub unsafe fn isr13();
    pub unsafe fn isr14();
    pub unsafe fn isr15();
    pub unsafe fn isr16();
    pub unsafe fn isr17();
    pub unsafe fn isr18();
    pub unsafe fn isr19();
    pub unsafe fn isr20();
    pub unsafe fn isr21();
    pub unsafe fn isr22();
    pub unsafe fn isr23();
    pub unsafe fn isr24();
    pub unsafe fn isr25();
    pub unsafe fn isr26();
    pub unsafe fn isr27();
    pub unsafe fn isr28();
    pub unsafe fn isr29();
    pub unsafe fn isr30();
    pub unsafe fn isr31();

    /// ISR functions for system calls.
    /// Eciton kernel syscall (0x66).
    pub unsafe fn isr102();
    pub unsafe fn isr128();
    pub unsafe fn isr177();

    /// IRQ functions for hardware interrupts 0-15.
    pub unsafe fn irq0();
    pub unsafe fn irq1();
    pub unsafe fn irq2();
    pub unsafe fn irq3();
    pub unsafe fn irq4();
    pub unsafe fn irq5();
    pub unsafe fn irq6();
    pub unsafe fn irq7();
    pub unsafe fn irq8();
    pub unsafe fn irq9();
    pub unsafe fn irq10();
    pub unsafe fn irq11();
    pub unsafe fn irq12();
    pub unsafe fn irq13();
    pub unsafe fn irq14();
    pub unsafe fn irq15();
}