// main.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello Rust OS!";

#[no_mangle] //No name mangling for this function
pub extern "C" fn _start() -> ! {
    // this func defines the entry point
    let vga_buffer = 0xb800 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This function is called on panic:
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
