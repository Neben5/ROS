#![macro_use]
use io::cpu_io;
use lazy_static::lazy_static;

pub mod io;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum TimeDate {
    Seconds = 0x00,
    Minutes = 0x02,
    Hours = 0x04,
    Weekday = 0x06,
    DOM = 0x07,
    Month = 0x08,
    Year = 0x09,
}

pub struct CmosTime {
    binary: bool,
    military: bool,
}

impl CmosTime {
    pub fn get(&self, register: TimeDate) -> u8 {
        unsafe {
            cpu_io::outb(0x70, 1 << 7 | register as u8);
            let mut val = cpu_io::inb(0x71);
            if !&self.binary {
                if register as u8 == TimeDate::Hours as u8 {
                    val = ((val & 0x0F) + (((val & 0x70) / 16) * 10)) | (val & 0x80);
                } else {
                    val = ((val & 0xF0) >> 1) + ((val & 0xF0) >> 3) + (val & 0xf);
                }
            }
            if (!&self.military) && (val & 0x80 != 0) {
                print!("{}", val & 0x80 != 0);

                val = ((val & 0x7F) + 12) % 24;
                // hopefully correct 12-hr conversion to 24-hr
            }
            return val;
        }
    }

    fn is_binary() -> bool {
        unsafe {
            if cpu_io::inb(0x0B) & 4 != 0 {
                return true;
            }
        }
        return false;
    }

    fn is_military() -> bool {
        unsafe {
            if !(cpu_io::inb(0x0B) & 2 != 0) {
                return false;
            }
            return true;
        }
    }
}
lazy_static! {
    pub static ref CMOS: CmosTime = CmosTime { // globally accessible
        binary: CmosTime::is_binary(),
        military: CmosTime::is_military(),
    };
}
