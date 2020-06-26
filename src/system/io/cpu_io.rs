use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref PORTS: Mutex<[u16;1024]> = Mutex::new([0;1024]); // note: i'm doing this because i feel that
                                                                    // it'd be really bad to have a usermoce
                                                                    // program interfering with kernel level
                                                                    // io bus ports
}

pub struct Port {
    port_number: u16,
}

impl Port {
    // TODO: implement port permissions
    pub unsafe fn new(port: u16) -> Port {
        return Port { port_number: port };
    }
    pub fn inb(&self) -> u8 {
        let result: u8;
        unsafe {
            llvm_asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(self.port_number) :: "volatile");
        }
        result
    }
    pub fn outb(&self, value: u8) {
        unsafe {
            llvm_asm!("outb %al, %dx" :: "{dx}"(self.port_number), "{al}"(value) :: "volatile");
        }
    }
    pub fn inw(&self) -> u16 {
        let result: u16;
        unsafe {
            llvm_asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(self.port_number) :: "volatile");
        }
        result
    }
    pub fn outw(&self, value: u16) {
        unsafe {
            llvm_asm!("outw %ax, %dx" :: "{dx}"(self.port_number), "{ax}"(value) :: "volatile");
        }
    }
    pub fn inl(&self) -> u32 {
        let result: u32;
        unsafe {
            llvm_asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(self.port_number) :: "volatile");
        }
        result
    }
    pub fn outl(&self, value: u32) {
        unsafe {
            llvm_asm!("outl %eax, %dx" :: "{dx}"(self.port_number), "{eax}"(value) :: "volatile");
        }
    }
}
