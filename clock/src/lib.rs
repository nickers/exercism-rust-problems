use std::fmt;

const CLOCK_MAX_MINUTES: i32 = 24*60;

pub struct Clock {
    total_minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{
            total_minutes: 0
        }.add_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock { 
            total_minutes: (self.total_minutes + (minutes % CLOCK_MAX_MINUTES) + CLOCK_MAX_MINUTES ) % CLOCK_MAX_MINUTES
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.total_minutes / 60, self.total_minutes % 60)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::cmp::PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.total_minutes == other.total_minutes
    }
}