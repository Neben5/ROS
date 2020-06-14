pub enum CMOS {
    Seconds,
    Minutes,
    Hours,
    Weekday,
    DOM,
    Month,
    Year,
}

pub fn read_cmos(register: CMOS) -> u8 {
    match register {
        CMOS::Seconds => 0x00,
        CMOS::Minutes => 0x02,
        CMOS::Hours => 0x04,
        CMOS::Weekday => 0x06,
        CMOS::DOM => 0x07, // Day of Month
        CMOS::Month => 0x08,
        CMOS::Year => 0x09,
    }
}
