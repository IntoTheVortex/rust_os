// main.rs
#![feature(prelude_2024)]

#![no_std]
#![no_main]


//testing
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]



mod vga_buffer;
//mod serial; 

use core::panic::PanicInfo;
//use rust_os::println;

//static HELLO: &[u8] = b"Hello Rust OS!";

// this func defines the entry point
#[no_mangle] //No name mangling for this function
pub extern "C" fn _start() -> ! {
    println!("Hello, it is Rust{}", "!");

    rust_os::init();
    //x86_64::instructions::interrupts::int3();

    /*
    //trigger page fault
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };
    */

    #[cfg(test)]
    test_main();

    //panic!("The Disco!");

    println!("It did not crash?!");
    loop {}
}

// This function is called on panic:
#[cfg(not(test))] //run this if not test run
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info); //why does the msg follow the location? change?
    loop {}
}

// our panic handler in test mode
// do we still need this one? after the one in lib
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

//test cases:
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
