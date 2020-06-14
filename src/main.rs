#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner::runner)] // test runnner is test_runner::runner
#![reexport_test_harness_main = "test_main"] // test_main() is now test entrypoint

mod vga_buffer;
use core::panic::PanicInfo;
mod system;
mod test_runner;
mod port;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // entrypoint
    #[cfg(test)]
    test_main(); // test

    print!("{}", system::read_cmos(system::CMOS::Seconds));
    loop {}
    // text mode cursor needs to be changed/disabled
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
