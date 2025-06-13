// SPDX-License-Identifier: GPL-3.0-or-later
// Date: 2025-06-13
// Author: Alexander Kuzin <alkuzindev@gmail.com>.

//! Architecture-specific code main module.

#[cfg(target_arch = "x86")]
pub mod x86;
