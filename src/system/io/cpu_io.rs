use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref PORTS: Mutex<[u16;1024]> = Mutex::new([0;1024]); // note: i'm doing this because i feel that
                                                                    // it'd be really bad to have a usermoce
                                                                    // program interfering with kernel level
                                                                    // io bus ports
}
pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    llvm_asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
    result
}
pub unsafe fn outb(port: u16, value: u8) {
    llvm_asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
}
pub unsafe fn inw(port: u16) -> u16 {
    let result: u16;
    llvm_asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(port) :: "volatile");
    result
}
pub unsafe fn outw(port: u16, value: u16) {
    llvm_asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(value) :: "volatile");
}
pub unsafe fn inl(port: u16) -> u32 {
    let result: u32;
    llvm_asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(port) :: "volatile");
    result
}
pub unsafe fn outl(port: u16, value: u32) {
    llvm_asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile");
}
