// main.rs
#![feature(prelude_2024)]

#![no_std]
#![no_main]

//testing
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]



mod vga_buffer;
mod serial; 

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
#[cfg(not(test))] //run this if not test run
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info); //why does the msg follow the location? change?
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

//testing
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    //serial_print!("trivial assertion! ");
    assert_eq!(1, 1);
    //serial_println!("[ok]");
}

// make test_runner print automatically
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}


// Exit Qemu:
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}