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

//! Contains functions for managing i686 registers.

use core::arch::asm;

/// CPU registers enumeration.
pub enum Register {
    /// Used for arithmetic operations and as an accumulator.
    Eax,
    /// Often used as a base pointer for data access.
    Ebx,
    /// Commonly used as a counter in loops and string operations.
    Ecx,
    /// Used for I/O operations and as a high-order accumulator.
    Edx,
    /// Typically used as a source index for string operations.
    Esi,
    /// Usually used as a destination index for string operations.
    Edi,
    /// Used as a base pointer for stack frames.
    Ebp,
    /// Points to the top of the stack.
    Esp,
    /// Holds the address of the next instruction to execute.
    Eip,
    /// Contains flags that control the operation of the CPU.
    Eflags,
    /// Used for data segment addressing.
    Ds,
    /// Used for extra segment addressing.
    Es,
    /// Used for additional segment addressing, often in thread-local storage.
    Fs,
    /// Used for additional segment addressing, often in thread-local storage.
    Gs,
    /// Holds the code segment selector.
    Cs,
    /// Holds the stack segment selector.
    Ss,
    /// Used for system control and configuration.
    Cr0,
    /// Holds the linear address that caused a page fault.
    Cr2,
    /// Holds the physical address of the page directory.
    Cr3,
}

/// CPU registers state struct.
pub struct RegisterState {
    /// The value of the EAX register.
    pub eax: u32,
    /// The value of the EBX register.
    pub ebx: u32,
    /// The value of the ECX register.
    pub ecx: u32,
    /// The value of the EDX register.
    pub edx: u32,
    /// The value of the ESI register.
    pub esi: u32,
    /// The value of the EDI register.
    pub edi: u32,
    /// The value of the EBP register.
    pub ebp: u32,
    /// The value of the ESP register.
    pub esp: u32,
    /// The value of the EIP register.
    pub eip: u32,
    /// The value of the EFLAGS register.
    pub eflags: u32,
    /// The value of the DS segment register.
    pub ds: u32,
    /// The value of the ES segment register.
    pub es: u32,
    /// The value of the FS segment register.
    pub fs: u32,
    /// The value of the GS segment register.
    pub gs: u32,
    /// The value of the CS segment register.
    pub cs: u32,
    /// The value of the SS segment register.
    pub ss: u32,
    /// The value of the CR0 control register.
    pub cr0: u32,
    /// The value of the CR2 control register.
    pub cr2: u32,
    /// The value of the CR3 control register.
    pub cr3: u32,
}

impl RegisterState {
    /// Create new instance of RegisterState.
    pub fn new() -> Self {
        RegisterState {
            eax:    read(Register::Eax),
            ebx:    read(Register::Ebx),
            ecx:    read(Register::Ecx),
            edx:    read(Register::Edx),
            esi:    read(Register::Esi),
            edi:    read(Register::Edi),
            ebp:    read(Register::Ebp),
            esp:    read(Register::Esp),
            eip:    read(Register::Eip),
            eflags: read(Register::Eflags),
            ds:     read(Register::Ds),
            es:     read(Register::Es),
            fs:     read(Register::Fs),
            gs:     read(Register::Gs),
            cs:     read(Register::Cs),
            ss:     read(Register::Ss),
            cr0:    read(Register::Cr0),
            cr2:    read(Register::Cr2),
            cr3:    read(Register::Cr3),
        }
    }
}

/// Get the specific register value.
///
/// # Parameters
/// - `reg` - given register name.
///
/// # Returns
/// Register value.
pub fn read(reg: Register) -> u32 {
    let ret: u32;

    unsafe {
        match reg {
            Register::Eax    => asm!("mov {0:e}, eax", out(reg) ret),
            Register::Ebx    => asm!("mov {0:e}, ebx", out(reg) ret),
            Register::Ecx    => asm!("mov {0:e}, ecx", out(reg) ret),
            Register::Edx    => asm!("mov {0:e}, edx", out(reg) ret),
            Register::Esi    => asm!("mov {0:e}, esi", out(reg) ret),
            Register::Edi    => asm!("mov {0:e}, edi", out(reg) ret),
            Register::Ebp    => asm!("mov {0:e}, ebp", out(reg) ret),
            Register::Esp    => asm!("mov {0:e}, esp", out(reg) ret),
            Register::Eip    => asm!("call 2f", "2: pop {0:e}", out(reg) ret),
            Register::Eflags => asm!("pushfd", "pop eax", out("eax") ret),
            Register::Ds     => asm!("mov {0:e}, ds", out(reg) ret),
            Register::Es     => asm!("mov {0:e}, es", out(reg) ret),
            Register::Fs     => asm!("mov {0:e}, fs", out(reg) ret),
            Register::Gs     => asm!("mov {0:e}, gs", out(reg) ret),
            Register::Cs     => asm!("mov {0:e}, cs", out(reg) ret),
            Register::Ss     => asm!("mov {0:e}, ss", out(reg) ret),
            Register::Cr0    => asm!("mov {0:e}, cr0", out(reg) ret),
            Register::Cr2    => asm!("mov {0:e}, cr2", out(reg) ret),
            Register::Cr3    => asm!("mov {0:e}, cr3", out(reg) ret),
        }
    }

    ret
}

/// Set the specific register.
///
/// # Parameters
/// - `reg`   - given register name.
/// - `value` - given register value to set.
pub fn write(reg: Register, value: u32) {
    unsafe {
        match reg {
            Register::Eax    => asm!("mov eax, {0:e}", in(reg) value),
            Register::Ebx    => asm!("mov ebx, {0:e}", in(reg) value),
            Register::Ecx    => asm!("mov ecx, {0:e}", in(reg) value),
            Register::Edx    => asm!("mov edx, {0:e}", in(reg) value),
            Register::Esi    => asm!("mov esi, {0:e}", in(reg) value),
            Register::Edi    => asm!("mov edi, {0:e}", in(reg) value),
            Register::Ebp    => asm!("mov ebp, {0:e}", in(reg) value),
            Register::Esp    => asm!("mov esp, {0:e}", in(reg) value),
            Register::Eip    => todo!("Not implemented"),
            Register::Eflags => asm!("push {0}", "popf", in(reg) value),
            Register::Ds     => asm!("mov ds, {0:e}", in(reg) value),
            Register::Es     => asm!("mov es, {0:e}", in(reg) value),
            Register::Fs     => asm!("mov fs, {0:e}", in(reg) value),
            Register::Gs     => asm!("mov gs, {0:e}", in(reg) value),
            Register::Cs     => asm!("mov cs, {0:e}", in(reg) value),
            Register::Ss     => asm!("mov ss, {0:e}", in(reg) value),
            Register::Cr0    => asm!("mov cr0, {0:e}", in(reg) value),
            Register::Cr2    => asm!("mov cr2, {0:e}", in(reg) value),
            Register::Cr3    => asm!("mov cr3, {0:e}", in(reg) value),
        }
    }
}

/// Eflags info struct.
pub struct Eflags {
    /// Eflags bit mask.
    pub mask: u32,
    /// Flag string representation.
    pub label: &'static str,
}

/// Number of flags.
pub const EFLAGS_COUNT: usize = 20;

/// Array of Eflags info structs.
pub const EFLAGS: [Eflags;EFLAGS_COUNT] = [
    Eflags {mask: 0x00000001, label: "CF"  }, // Carry flag.
    Eflags {mask: 0x00000004, label: "PF"  }, // Parity flag.
    Eflags {mask: 0x00000010, label: "AF"  }, // Auxiliary flag.
    Eflags {mask: 0x00000040, label: "ZF"  }, // Zero flag.
    Eflags {mask: 0x00000080, label: "SF"  }, // Sign flag.
    Eflags {mask: 0x00000100, label: "TF"  }, // Trap flag.
    Eflags {mask: 0x00000200, label: "IF"  }, // Interrupt enable flag.
    Eflags {mask: 0x00000400, label: "DF"  }, // Direction flag.
    Eflags {mask: 0x00000800, label: "OF"  }, // Overflow flag.
    Eflags {mask: 0x00003000, label: "IOPL"}, // I/O privilege level.
    Eflags {mask: 0x00004000, label: "NT"  }, // Nested task flag.
    Eflags {mask: 0x00008000, label: "MD"  }, // Mode flag.
    Eflags {mask: 0x00010000, label: "RF"  }, // Resume flag.
    Eflags {mask: 0x00020000, label: "VM"  }, // Virtual 8086 mode flag.
    Eflags {mask: 0x00040000, label: "AC"  }, // Alignment check.
    Eflags {mask: 0x00080000, label: "VIF" }, // Virtual interrupt flag.
    Eflags {mask: 0x00100000, label: "VIP" }, // Virtual interrupt pending.
    Eflags {mask: 0x00200000, label: "ID"  }, // Able to use CPUID.
    Eflags {mask: 0x40000000, label: "AES" }, // AES key schedule loaded.
    Eflags {mask: 0x80000000, label: "AI"  }  // Alternate Instruction Set.
];