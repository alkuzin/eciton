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

//! Eciton exokernel syscall API main module.

#![allow(unused_imports)]

mod allocpg;
mod freepg;
mod getfb;

pub use allocpg::allocpg;
pub use freepg::freepg;
pub use getfb::getfb;
use core::arch::asm;

// TODO: make syscalls compatible with C.

/// Eciton exokernel syscall interrupt number.
const SYSCALL_NUM: u32 = 0x66;

/// Syscall arguments structure.
#[derive(Debug, Default)]
struct SyscallArgs {
    /// Argument value stored in EAX register.
    arg1: u32,
    /// Argument value stored in EBX register.
    arg2: u32,
    /// Argument value stored in ECX register.
    arg3: u32,
    /// Argument value stored in EDX register.
    arg4: u32,
    /// Argument value stored in ESI register.
    arg5: u32,
    /// Argument value stored in EDI register.
    arg6: u32,
}

/// Make system call.
///
/// # Parameters
/// - `args` - given syscall arguments.
///
/// # Returns
/// - Syscall output.
fn syscall(args: &SyscallArgs) -> SyscallArgs {
    let mut ret = SyscallArgs::default();

    unsafe {
        // Write syscall arguments in corresponding registers.
        asm!(
            "mov eax, {0:e}",
            "mov ebx, {1:e}",
            "mov ecx, {2:e}",
            "mov edx, {3:e}",
            "mov edi, {4:e}",
            "mov esi, {5:e}",
            "int {6}",
            in(reg) args.arg1,
            in(reg) args.arg2,
            in(reg) args.arg3,
            in(reg) args.arg4,
            in(reg) args.arg5,
            in(reg) args.arg6,
            const SYSCALL_NUM,
        );

        // Read syscall arguments from corresponding registers.
        asm!(
            "mov {0:e}, eax",
            "mov {1:e}, ebx",
            "mov {2:e}, ecx",
            "mov {3:e}, edx",
            "mov {4:e}, edi",
            "mov {5:e}, esi",
            out(reg) ret.arg1,
            out(reg) ret.arg2,
            out(reg) ret.arg3,
            out(reg) ret.arg4,
            out(reg) ret.arg5,
            out(reg) ret.arg6,
        );
    }

    ret
}

/// Syscall numbers enumeration.
#[repr(u32)]
pub enum Syscall {
    /// Get framebuffer syscall number.
    Getfb = 1,
    /// Alloc memory pages syscall number.
    AllocPg = 2,
    /// Free memory pages syscall number.
    FreePg = 3,
}