use std::fmt;

const DAY: i64 = 1_440; // one day  = 24 hours = 1440 (24 * 60) minutes

const HOUR: i64 = 60; // one hour = 60 mins
#[derive(Debug, Eq, PartialEq)]

pub struct Clock {
    mins: i64,
}

// macro calculates mins from mins and hours

macro_rules! calc_mins {
    ($m:expr,$h:expr) => {{
        ((($h * HOUR + $m) % DAY) + DAY) % DAY
    }};
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Clock {
        Clock {
            mins: calc_mins!(minutes, hours),
        }
    }

    pub fn add_minutes(self, minutes: i64) -> Clock {
        Clock::new(0, self.mins + minutes)
    }
}

// implemenation of ::Display for Clock

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.mins / HOUR, self.mins % HOUR)
    }
}
