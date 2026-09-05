use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock{
    minutes: i32,
}

const MINS_IN_DAY: i32 = 60*24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * 60 + minutes).rem_euclid(MINS_IN_DAY);

        Self {
            minutes: total_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, &self.minutes + minutes)
    }

    pub fn subtract_minutes(&self, minutes: i32) -> Self {
                Self::new(0, &self.minutes - minutes)

    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hour = &self.minutes / 60;
        let minutes = &self.minutes % 60;
        write!(f, "{hour:02}:{minutes:02}")
    }
}