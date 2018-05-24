use std::time::{SystemTime, UNIX_EPOCH};

pub enum Month{
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
    None,
}
impl Month{
    fn getMonth(days: u32, isLeapYear: bool) -> (Month, i32){
        let (m:Month, da: u32) = if !isLeapYear {
            match days{
                1..32    => (Month::January,   days      ),
                32..60   => (Month::February,  days -  31),
                60..91   => (Month::March,     days -  59),
                91..121  => (Month::April,     days -  90),
                121..152 => (Month::Mai,       days - 120),
                152..182 => (Month::June,      days - 151),
                182..213 => (Month::July,      days - 181),
                213..244 => (Month::August,    days - 212),
                244..273 => (Month::September, days - 243),
                273..305 => (Month::October,   days - 272),
                305..335 => (Month::November,  days - 304),
                335..366 => (Month::December,  days - 334),
            } 
        } else {
             match days{
                1..32    => (Month::January,   days      ),
                32..61   => (Month::February,  days -  31),
                61..92   => (Month::March,     days -  60),
                92..122  => (Month::April,     days -  91),
                122..153 => (Month::Mai,       days - 121),
                153..183 => (Month::June,      days - 152),
                183..214 => (Month::July,      days - 182),
                214..245 => (Month::August,    days - 213),
                245..274 => (Month::September, days - 244),
                274..306 => (Month::October,   days - 273),
                306..336 => (Month::November,  days - 305),
                336..367 => (Month::December,  days - 335),
            } 
        }
    (m, da)
    }
}
pub struct Time{
    pub year:          i32,
    pub total_days:    i32,
    pub month:       Month,
    pub day:            i8,
    pub hour:           i8,
    pub minute:         i8,
    pub seconds:        i8,
    pub seconds_raw:   u64,
    pub nanoseconds:   u32,
}
impl Time{
    pub fn now() -> Time {
        let mut Time = {
            year:                1970,
            total_days:             0,
            month:        Month::None,
            day:                    0,
            hour:                   0,
            minute:                 0,
            seconds:                0,
            seconds_raw:            0,
            nanoseconds:            0,
        };
        match SystemTime::now().duration_since(UNIX_EPOCH){
            Ok(n) => {
                Time.seconds_raw = n.as_secs();
                Time.nanoseconds = n.subsec_nanos();
            }
            Err(_) => continue,
        }
        let mut sec: u64 = Time.seconds_raw;
        Time.seconds = sec % 60; // calc to seconds
        sec /= 60;
        Time.minute = sec % 60; // calc the minutes
        sec /= 60;
        Time.hour = sec % 24; // calc the minutes
        sec /= 24;
        let mut days: i32 = sec;
        loop {
            if days % 4 == 0 && (( ! days % 100 == 0) || (days % 100 == 0 && days % 400 == 0)){
                if days > 366 {
                    days -= 366;
                    Time.year += 1;
                } else {
                    break;
                }
            } else {
                if days > 365 {
                    days -= 365;
                    Time.year += 1;
                } else {
                    break;
                }
            }
        }
    }
}
