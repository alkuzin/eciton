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

//! Kernel syscalls module.

use crate::{
    eciton::arch::i686::irq::{self, IntRegisterState},
    pr_debug,
};

pub const SYSCALL_NUM: usize = 0x66;
const SYSCALL_COUNT: usize   = 1;

/// Get framebuffer syscall number.
const SYSCALL_GETFB: u32 = 1;

// TODO: add syscall table

/// Initialize syscall handler.
pub fn init() {
    irq::request(SYSCALL_NUM, syscall_handler);
}

/// Handle syscalls.
///
/// # Parameters
/// - `regs` - given pointer to interrupt register state.
fn syscall_handler(regs: &IntRegisterState) {
    pr_debug!("SYSCALL HANDLER BEGIN");
    pr_debug!("{:#X?}", regs);

    match regs.eax {
        0             => pr_debug!("?"),
        SYSCALL_GETFB => sys::getfb(regs),
        _ => {},
    }

    pr_debug!("SYSCALL HANDLER END");
}

// TODO: move to separate module.
pub mod sys {
    use crate::{
        eciton::arch::i686::{
            irq::IntRegisterState,
            register::{self, Register}
        },
        sdk::Framebuffer,
    };

    /// <SYSCALL> Get framebuffer.
    ///
    /// # Parameters
    /// - `regs` - given pointer to interrupt register state.
    pub fn getfb(regs: &IntRegisterState) {
        let boot_info = crate::BOOT_INFO.lock();
        let fb_ptr    = regs.ebx as *mut Framebuffer;

        // Check that given framebuffer info pointer is valid.
        if fb_ptr.is_null() {
            // Put error return value -1 into eax register.
            register::write(Register::Eax, u32::MAX);
            return;
        }

        // Fill framebuffer info.
        unsafe {
            let fb    =  &mut *fb_ptr;
            fb.addr   = boot_info.framebuffer_addr;
            fb.pitch  = boot_info.framebuffer_pitch;
            fb.width  = boot_info.framebuffer_width;
            fb.height = boot_info.framebuffer_height;
            fb.bpp    = boot_info.framebuffer_bpp;
        }

        // Put success return value into eax register.
        register::write(Register::Eax, 0);
    }
}