#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#[allow(dead_code)]
#[allow(unused_must_use)]

use core::panic::PanicInfo;
mod vga_buffer;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // entrypoint
    println!("hey there");
    let mut i = true;
    print!("|{}|", 0x20 as char);
    print!("|{}|", 0x7e as char); // no default toString lol
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
