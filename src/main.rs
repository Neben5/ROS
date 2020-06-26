#![allow(dead_code)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(llvm_asm)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner::runner)] // test runnner is test_runner::runner
#![reexport_test_harness_main = "test_main"] // test_main() is now test entrypoint
#![allow(unconditional_panic)]
#![allow(const_err)]

mod vga_buffer;
use core::panic::PanicInfo;
mod system;
mod test_runner;
use system::TimeDate;

// TODO: need to add exceptions >> paging >> virt mem >> fs >> usermode >> basic terminal >> proper vga

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // entrypoint
    #[cfg(test)]
    test_main(); // test
    let mut val = System!().time.get(TimeDate::Seconds);
    loop {
        let temp = System!().time.get(TimeDate::Seconds);
        if temp != val {
            val = temp;
            println!("{} : {}", System!().time.get(TimeDate::Minutes), val);
        }
    }
    // text mode cursor needs to be changed/disabled
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    println!("Reloading kernel");
    _start();
}

#[test_case]
fn trivial_assertion() {
    serial_println!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}
