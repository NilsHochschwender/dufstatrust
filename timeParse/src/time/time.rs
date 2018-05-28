use std::time::{SystemTime, UNIX_EPOCH};
extern crate regex;
#[macro_use] extern crate lazy_static;
use regex::Regex;

macro_rules! str {
   ($(x:expr), *) => (
       let mut string_tmp = String::new();
       $(
           string_tmp.push_str(String::from($x));
       )*
       string_tmp
    )
}


#[derive(Debug)]
pub enum Month {
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
    pub fn get_short(&self) -> String {
        match *self {
            January   => str!("Jan"),
            February  => str!("Feb"),
            March     => str!("Mar"),
            April     => str!("Apr"),
            May       => str!("May"),
            June      => str!("Jun"),
            July      => str!("Jul"),
            August    => str!("Aug"),
            September => str!("Sep"),
            October   => str!("Oct"),
            November  => str!("Nov"),
            December  => str!("Dec"),
            None | _  => str!(""),
        }
    }
    pub fn get_long(&self) -> String {
        match *self {
            January   => str!("January"),
            February  => str!("February"),
            March     => str!("March"),
            April     => str!("April"),
            May       => str!("May"),
            June      => str!("June"),
            July      => str!("July"),
            August    => str!("August"),
            September => str!("September"),
            October   => str!("October"),
            November  => str!("November"),
            December  => str!("December"),
            None | _  => str!(""),
        }
    }
    pub fn get_int(&self) -> i32 {
        match *self {
            January   => 1,
            February  => 2,
            March     => 3,
            April     => 4,
            May       => 5,
            June      => 6,
            July      => 7,
            August    => 8,
            September => 9,
            October   => 10,
            November  => 11,
            December  => 12,
            None | _  => 0,
        }
    }
}


fn get_month(days: i32, isLeapYear: bool) -> (Month, i8){
    let (m, da) = if !isLeapYear {
        match days{
            1...31    => (Month::January,   days      ),
            32...59   => (Month::February,  days -  31),
            60...90   => (Month::March,     days -  59),
            91...120  => (Month::April,     days -  90),
            121...151 => (Month::May,       days - 120),
            152...181 => (Month::June,      days - 151),
            182...212 => (Month::July,      days - 181),
            213...243 => (Month::August,    days - 212),
            244...272 => (Month::September, days - 243),
            273...304 => (Month::October,   days - 272),
            305...334 => (Month::November,  days - 304),
            335...365 => (Month::December,  days - 334),
            _         => (Month::None,               0),
        }
    } else {
        match days{
            1...31    => (Month::January,   days      ),
            32...60   => (Month::February,  days -  31),
            61...91   => (Month::March,     days -  60),
            92...121  => (Month::April,     days -  91),
            122...152 => (Month::May,       days - 121),
            153...182 => (Month::June,      days - 152),
            183...213 => (Month::July,      days - 182),
            214...244 => (Month::August,    days - 213),
            245...273 => (Month::September, days - 244),
            274...305 => (Month::October,   days - 273),
            306...335 => (Month::November,  days - 305),
            336...366 => (Month::December,  days - 335),
            _         => (Month::None,               0),
        }
    };
    (m, da as i8)
}

#[derive(Debug)]
pub enum Day{
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
    None,
}
impl Day{
    pub fn get_short(&self) -> String {
        match *self {
            Mon      => str!("Mon"),
            Tue      => str!("Tue"),
            Wed      => str!("Wed"),
            Thu      => str!("Thu"),
            Fri      => str!("Fri"),
            Sat      => str!("Sat"),
            Sun      => str!("Sun"),
            None | _ => str!(""),
        }
    }
    pub fn get_long(&self) -> String {
        match *self {
            Mon      => str!("Monday"),
            Tue      => str!("Tuesday"),
            Wed      => str!("Wednesday"),
            Thu      => str!("Thursday"),
            Fri      => str!("Friday"),
            Sat      => str!("Saturday"),
            Sun      => str!("Sunday"),
            None | _ => str!(""),
        }
    }
    pub fn get_int(&self) -> i32 {
        match *self {
            Mon      => 1,
            Tue      => 2,
            Wed      => 3,
            Thu      => 4,
            Fri      => 5,
            Sat      => 6,
            Sun      => 7,
            None | _ => 0,
        }
    }
}

#[derive(Debug)]
pub struct Time{
    year: i32,
    total_days:    i32,
    week:           i8,
    day_str:       Day,
    month:       Month,
    day:            i8,
    hour:           i8,
    minute:         i8,
    seconds:        i8,
    seconds_raw:   u64,
    nanoseconds:   u32,
}
impl Time{
    pub fn now() -> Time {
        let mut time: Time = Time{
            year:                1970i32,
            total_days:             0i32,
            week:                    0i8,
            day_str:           Day::None,
            month:           Month::None,
            day:                     0i8,
            hour:                    0i8,
            minute:                  0i8,
            seconds:                 0i8,
            seconds_raw:            0u64,
            nanoseconds:            0u32,
        };
        match SystemTime::now().duration_since(UNIX_EPOCH){
            Ok(n) => {
                time.seconds_raw = n.as_secs();
                time.nanoseconds = n.subsec_nanos();
            }
            Err(_) => {},
        }
        let mut sec: u64 = time.seconds_raw;
        time.seconds = (sec % 60) as i8; // calc to seconds
        sec /= 60;
        time.minute = (sec % 60) as i8; // calc the minutes
        sec /= 60;
        time.hour = (sec % 24) as i8; // calc the minutes
        sec /= 24;
        let mut days: i32 = sec as i32;
        let is_leap_year = |y: i32| -> bool {y % 4 == 0 && (( ! y % 100 == 0) || (y % 100 == 0 && y % 400 == 0))};
        loop {
            let year = time.year + 1;
            if is_leap_year(year){
                if days > 366 {
                    days -= 366;
                    time.year += 1;
                } else {
                    break;
                }
            } else {
                if days > 365 {
                    days -= 365;
                    time.year += 1;
                } else {
                    break;
                }
            }
        }
        time.total_days = days;
        let (month, day) = get_month(time.total_days, is_leap_year(time.year));
        time.month = month;
        time.day = day;
        let day_in_week = |&m: Month, d: i32, y: i32| -> i32 {
            let m = match m.Month{
                January   => 1,
                February  => 2,
                March     => 3,
                April     => 4,
                May       => 5,
                June      => 6,
                July      => 7,
                August    => 8,
                September => 9,
                October   => 10,
                November  => 11,
                December  => 12,
                None | _  => -1,
            };
            let mut w = 0i32;
            if m != -1 {
                let y = if m < 3 {y -1} else {y};
                w = (((d as f64 + ((2.6f64 * ((m + 9) % 12 +1) as f64) as f64).floor() \
                    + y % 100 + ((y as f64 % 100f64 / 4f64) as f64).floor() \
                    + ((y as f64 / 400f64) as f64).floor() - 2 * ((y as f64 / 100f64) as f64).floor() - 1) % 7 + 7) % 7 + 1) as i32;
            }
            w
        }
        time.day_str = match day_in_week(&time.month, time.day, time.year){
                1 => Day::Mon,
                2 => Day::Tue,
                3 => Day::Wed,
                4 => Day::Thu,
                5 => Day::Fri,
                6 => Day::Sat,
                7 => Day::Sun,
                _ => Day::None,
            }
        }
        time
    }
    pub fn format_as_string(self, form: str) -> Option<String> {
        //  %%   a literal %
        //  %a   locale's abbreviated weekday name (e.g., Sun)
        //  %A   locale's full weekday name (e.g., Sunday)
        //  %b   locale's abbreviated month name (e.g., Jan)
        //  %B   locale's full month name (e.g., January)
        //  %c   locale's date and time (e.g., Thu Mar  3 23:05:25 2005)
        //  %C   century; like %Y, except omit last two digits (e.g., 20)
        //  %d   day of month (e.g., 01)
        //  %D   date; same as %m/%d/%y
        //  %e   day of month, space padded; same as %_d
        //  %F   full date; same as %Y-%m-%d
        //  %g   last two digits of year of ISO week number (see %G)
        //  %G   year of ISO week number (see %V); normally useful only with %V
        //  %h   same as %b
        //  %H   hour (00..23)
        //  %I   hour (01..12)
        //  %j   day of year (001..366)
        //  %k   hour, space padded ( 0..23); same as %_H
        //  %l   hour, space padded ( 1..12); same as %_I
        //  %m   month (01..12)
        //  %M   minute (00..59)
        //  %n   a newline
        //  %N   nanoseconds (000000000..999999999)
        //  %p   locale's equivalent of either AM or PM; blank if not known
        //  %P   like %p, but lower case
        //  %r   locale's 12-hour clock time (e.g., 11:11:04 PM)
        //  %R   24-hour hour and minute; same as %H:%M
        //  %s   seconds since 1970-01-01 00:00:00 UTC
        //  %S   second (00..60)
        //  %t   a tab
        //  %T   time; same as %H:%M:%S
        //  %u   day of week (1..7); 1 is Monday
        //  %U   week number of year, with Sunday as first day of week (00..53)
        //  %V   ISO week number, with Monday as first day of week (01..53)
        //  %w   day of week (0..6); 0 is Sunday
        //  %W   week number of year, with Monday as first day of week (00..53)
        //  %x   locale's date representation (e.g., 12/31/99)
        //  %X   locale's time representation (e.g., 23:13:48)
        //  %y   last two digits of year (00..99)
        //  %Y   year
        //  %z   +hhmm numeric time zone (e.g., -0400)
        //  %:z  +hh:mm numeric time zone (e.g., -04:00)
        //  %::z  +hh:mm:ss numeric time zone (e.g., -04:00:00)
        //  %:::z  numeric time zone with : to necessary precision (e.g., -04, +05:30)
        //  %Z   alphabetic time zone abbreviation (e.g., EDT)
        let re = Regex::new(r"%[_]{0,1}[%aAbBcCdDeFgGhHIjklmMnNpPrRsStTuUVwWxXyYzZ][:]{0,3}.").unwrap();
        let mut formssplit: Vec<String>;
        for mat in re.find_iter(form){
            formssplit.push(String::From(mat.as_str));
        }
        let mut out: String;
        for i in formssplit.iter(){
            let mut substring: String = i.clone();
            let _ = substring.remove(0);
            let last = substring.pop();
            match substring{
                "%"        =>  out.push_str(str!("%", last)),
                "a"        =>  out.push_str(str!(self.day_str.get_short(), last)),
                "A"        =>  out.push_str(str!(self.day_str.get_long(), last)),
                "b" | "h"  =>  out.push_str(str!(self.month.get_short(), last)),
                "B"        =>  out.push_str(str!(self.month.get_long(), last)),
                "c"        =>  out.push_str(str!(self.day_str.get_short(), " ", self.month.get_short(), " ", self.days, " ", self.hour, ":", self.minute, ":", self.seconds, " ", self.year, last)),
                "C"        =>  out.push_str(str!(self.year / 100, last)),
                "d"        =>  out.push_str(str!(self.days, last)),
                "D"        =>  out.push_str(str!(self.month.get_int(), "/", self.day_str.get_int(), "/", self.year % 100, last)),
                "e" | "_d" =>  out.push_str(str!(String::from(self.days).replace("0"," "), last)),
                "F"        =>  out.push_str(str!(self.year, "-", self.month.get_int(), "-", self.days, last)),
                "H"        =>  out.push_str(str!(self.hour, last)),
                "I"        =>  out.push_str(str!(self.hour % 12 + 1, last)),
                "j"        =>  out.push_str(str!(self.total_days, last)),
                "k" | "_H" =>  out.push_str(str!(String::from(self.hour).replace("0", " "), last)),
                "l" | "_I" =>  out.push_str(str!(String::from(self.hour % 12 + 1).replace("0", " "), last)),
                "m"        =>  out.push_str(str!(self.month.get_int(), last)),
                "M"        =>  out.push_str(str!(self.minute, last)),
                "n"        =>  out.push_str(str!("\n")),
                "N"        =>  out.push_str(str!(self.nanoseconds, last)),
                "r"        =>  out.push_str(str!(self.hour % 12 , ":", self.minute, ":", self.seconds, " ", if self.hour > 11{"AM"} else {"PM"}, last)),
                "R"        =>  out.push_str(str!())
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn create_new_date(){
        let time = Time::now();
        println!("{:?}", time);
    }
}
