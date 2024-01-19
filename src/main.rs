// main.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;


#[no_mangle] //No name mangling for this function
pub extern "C" fn _start() -> ! {
    // this func defines the entry point
    loop {}
}

/// This function is called on panic:
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
