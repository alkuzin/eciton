
#![no_std]                      // Do not use the standard library.
#![no_main]                     // Do not use the standard main function.
#![allow(clippy::empty_loop)]   // Ignore empty loop.
#![allow(dead_code)]            // Allow unused values.

use core::panic::PanicInfo;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}