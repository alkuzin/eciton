// Project name: Eciton.
// Description: Experimental exokernel.
// Licence: GPL-3.0.
// Author: Alexander (@alkuzin).

//! Main exokernel module. Responsible for initializing kernel components.

use crate::{log, multiboot::MultibootInfo};

/// Initialize kernel.
///
/// # Parameters
/// - `boot_info` - given multiboot info structure.
pub fn init(_boot_info: &MultibootInfo) {
    // Initialize kernel logger.
    log::init();
}
