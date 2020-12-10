#[macro_use]
use crate::system::io::cpu_io;
use crate::serial_println;

#[cfg(test)]
pub fn runner(tests: &[&dyn Fn()]) {
    // tests is a reference to a slice of objects with trait 'Fn()'
    // basically a list of references to types that can be called like a function
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        cpu_io::outb(0xf4, exit_code as u8);
    }
}
