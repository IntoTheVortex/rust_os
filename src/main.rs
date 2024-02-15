// main.rs
#![feature(prelude_2024)]
// use core::prelude::rust_2024::derive;
#![no_std]
#![no_main]



mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello Rust OS!";

#[no_mangle] //No name mangling for this function
pub extern "C" fn _start() -> ! {
    // this func defines the entry point
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello RUST").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}

/// This function is called on panic:
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
