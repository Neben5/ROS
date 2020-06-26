use crate::system::io;
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
    let port = unsafe { io::cpu_io::Port::new(0xf4) };
    port.outb(exit_code as u8);
}
