use io::cpu_io::Port;
use lazy_static::lazy_static;
use spin::Mutex; // prevents races for writer // global static objects, writer
pub mod io;

pub struct CMOS {
    wport: Port,
    rport: Port,
}

#[repr(u16)]
pub enum TimeDate {
    Seconds = 0x00,
    Minutes = 0x02,
    Hours = 0x04,
    Weekday = 0x06,
    DOM = 0x07,
    Month = 0x08,
    Year = 0x09,
}

impl CMOS {
    fn new() -> CMOS {
        return CMOS {
            wport: unsafe { Port::new(0x70) },
            rport: unsafe { Port::new(0x71) },
        };
    }

    pub fn get(&self, register: TimeDate) -> u8 {
        self.wport.outb(0x80 | register as u8);
        return self.rport.inb();
    }
}

pub struct Sys{
    pub time: CMOS,
}

lazy_static!{
    pub static ref SYSTEM: Mutex<Sys> = Mutex::new(Sys{
        time: CMOS::new(),
    });
}

#[macro_export]
macro_rules! System {
    () => {
        $crate::system::SYSTEM.lock();
    };
}