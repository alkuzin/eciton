#![no_main]
#![no_std]

use core::panic::PanicInfo;

static GREETING: &[u8] = b"Eciton exokernel v0.0.0";

#[unsafe(no_mangle)]
extern "C" fn kmain() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in GREETING.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xE;
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}