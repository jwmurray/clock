// use std::cmp::Ordering;
use std::fmt;

const MINUTES_PER_HOUR: i32 = 60;
const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_DAY: i32 = MINUTES_PER_HOUR * HOURS_PER_DAY;

#[derive(PartialEq, Debug)]
pub struct Clock {
    total_minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // let internal_hours = match hours.cmp(&0) {
        //     Ordering::Less => HOURS_PER_DAY - ((hours * -1) % HOURS_PER_DAY),
        //     _ => hours % HOURS_PER_DAY,
        // };
        // let mut internal_minutes = minutes;
        // while internal_minutes > MINUTES_PER_HOUR || internal_minutes < -1 * MINUTES_PER_HOUR {
        //     internal_minutes %= MINUTES_PER_HOUR;
        // }
        // Clock {
        //     total_minutes: internal_hours * MINUTES_PER_HOUR + internal_minutes,
        // }
        // let mut total_minutes =
        //     internal_hours % HOURS_PER_DAY * MINUTES_PER_HOUR + internal_minutes;

        // let mut total_minutes = hours * MINUTES_PER_HOUR + minutes;
        // while total_minutes < 0 {
        //     if total_minutes < MINUTES_PER_DAY {
        //         total_minutes += MINUTES_PER_DAY
        //     } else {
        //         total_minutes += MINUTES_PER_HOUR;
        //     }
        // }
        // while total_minutes >= MINUTES_PER_DAY {
        //     total_minutes -= MINUTES_PER_DAY;
        // }
        let clock = Clock { total_minutes: 0 };
        clock.add_hours(hours);
        println!("clock: {:?}", clock);
        clock.add_minutes(minutes);
        clock
        // Clock { total_minutes }
    }

    fn add_hours(&self, hours: i32) -> Self {
        let mut hours_to_add = hours;
        while hours_to_add < 0 {
            hours_to_add += HOURS_PER_DAY;
        }
        while hours_to_add >= HOURS_PER_DAY {
            hours_to_add -= HOURS_PER_DAY;
        }
        println!("hours_to_add: {:?}", hours_to_add);
        self.add_minutes(hours_to_add * MINUTES_PER_HOUR)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut final_minutes = minutes + self.total_minutes;
        while final_minutes < 0 {
            if final_minutes < MINUTES_PER_DAY {
                final_minutes += MINUTES_PER_DAY
            } else {
                final_minutes += MINUTES_PER_HOUR;
            }
        }
        while final_minutes >= MINUTES_PER_DAY {
            final_minutes -= MINUTES_PER_DAY;
        }

        let return_clock = Clock {
            total_minutes: final_minutes,
        };
        println!("{:?}", return_clock);
        return_clock
    }

    fn hours(&self) -> i32 {
        (self.total_minutes / MINUTES_PER_HOUR) % HOURS_PER_DAY
    }

    fn minutes(&self) -> i32 {
        self.total_minutes % MINUTES_PER_HOUR
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}
