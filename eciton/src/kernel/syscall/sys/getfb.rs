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

//! Get framebuffer syscall implementation.

use super::{IntRegisterState, SyscallResult};
use eciton_sdk::vbe::Framebuffer;

/// Get framebuffer.
///
/// # Parameters
/// - `regs` - given pointer to interrupt register state.
pub fn getfb(regs: &mut IntRegisterState) {
    let boot_info = crate::BOOT_INFO.lock();
    let fb_ptr    = regs.ebx as *mut Framebuffer;

    // Check that given framebuffer info pointer is valid.
    if fb_ptr.is_null() {
        // Put error return value -1 into eax register.
        regs.eax = SyscallResult::Error as u32;
        return;
    }

    // Fill framebuffer info.
    unsafe {
        let fb    = &mut *fb_ptr;
        fb.addr   = boot_info.framebuffer_addr;
        fb.pitch  = boot_info.framebuffer_pitch;
        fb.width  = boot_info.framebuffer_width;
        fb.height = boot_info.framebuffer_height;
        fb.bpp    = boot_info.framebuffer_bpp;
    }

    // Put success return value into eax register.
    regs.eax = SyscallResult::Success as u32;
}

use crate::tests::*;

exotest! {
    exotest_test_cases! {
        test_getfb_syscall_successfull, {
            let mut regs = IntRegisterState::default();
            let fb       = Framebuffer::default();

            regs.ebx = (&fb as *const _) as u32;
            getfb(&mut regs);

            let fb_ptr = regs.ebx as *const Framebuffer;
            let fb     = unsafe { &*fb_ptr };

            // Check that framebuffer info is correct.
            let boot_info = crate::BOOT_INFO.lock();

            assert_eq!(fb.addr,   boot_info.framebuffer_addr);
            assert_eq!(fb.pitch,  boot_info.framebuffer_pitch);
            assert_eq!(fb.width,  boot_info.framebuffer_width);
            assert_eq!(fb.height, boot_info.framebuffer_height);
            assert_eq!(fb.bpp,    boot_info.framebuffer_bpp);

            // Check return value.
            let ret = regs.eax;
            assert_eq!(ret, SyscallResult::Success as u32);
        },

        test_getfb_syscall_incorrect, {
            let mut regs = IntRegisterState::default();
            regs.ebx = 0;
            getfb(&mut regs);

            // Check return value.
            let ret = regs.eax;
            assert_eq!(ret, SyscallResult::Error as u32);
        }
    }
}