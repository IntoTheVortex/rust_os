// main.rs
#![feature(prelude_2024)]

#![no_std]
#![no_main]

//testing
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]



mod vga_buffer;

use core::panic::PanicInfo;

//static HELLO: &[u8] = b"Hello Rust OS!";

#[no_mangle] //No name mangling for this function
pub extern "C" fn _start() -> ! {
    // this func defines the entry point
    println!("Hello, it is Rust{}", "!");

    #[cfg(test)]
    test_main();

    //panic!("The Disco!");

    loop {}
}

/// This function is called on panic:
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info); //why does the msg follow the location? change?
    loop {}
}

//testing
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion! ");
    assert_eq!(1, 1);
    println!("[ok]");
}