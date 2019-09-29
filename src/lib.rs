use std::cmp::Ordering;
use std::fmt;

const MINUTES_PER_HOUR: u32 = 60;
const HOURS_PER_DAY: u32 = 24;
const MINUTES_PER_DAY: u32 = MINUTES_PER_HOUR * HOURS_PER_DAY;

#[derive(PartialEq, Debug)]
pub struct Clock {
    total_minutes: u32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours_u32 = match hours.cmp(&0) {
            Ordering::Less => (HOURS_PER_DAY as i32 + hours) as u32,
            _ => hours as u32 % HOURS_PER_DAY,
        };

        let minutes_u32: u32 = match minutes.cmp(&0) {
            Ordering::Less => {
                let mut pos_minutes = minutes;
                while pos_minutes < -(MINUTES_PER_HOUR as i32) {
                    pos_minutes += MINUTES_PER_DAY as i32
                }
                while minutes < 0 {
                    pos_minutes += MINUTES_PER_HOUR as i32;
                }
                pos_minutes as u32
            }
            _ => {
                let mut pos_minutes = minutes as u32 % MINUTES_PER_DAY;
                while pos_minutes > MINUTES_PER_HOUR {
                    pos_minutes -= MINUTES_PER_HOUR;
                }
                pos_minutes
            }
        };
        let total_minutes = hours_u32 * MINUTES_PER_HOUR + minutes_u32;
        Self { total_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
    fn hours(&self) -> u32 {
        (self.total_minutes / MINUTES_PER_HOUR) % HOURS_PER_DAY
    }

    fn minutes(&self) -> u32 {
        self.total_minutes % MINUTES_PER_HOUR
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}
