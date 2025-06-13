// SPDX-License-Identifier: GPL-3.0-or-later
// Date: 2025-06-13
// Author: Alexander Kuzin <alkuzindev@gmail.com>.

//! HAL (Hardware Abstraction Layer) main module.

use crate::arch;

pub mod cpu;
pub mod uart;
pub mod keyboard;

/// Initialize architecture-specific part of the kernel.
pub fn init() {
    #[cfg(target_arch = "x86")]
    arch::x86::init();
}
