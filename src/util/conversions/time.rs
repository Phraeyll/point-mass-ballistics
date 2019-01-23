use crate::util::Numeric;
use Time::*;

pub const HOURS_TO_MINUTES: Numeric = 60.0;
pub const MINUTES_TO_HOURS: Numeric = 1.0 / HOURS_TO_MINUTES;

pub const MINUTES_TO_SECONDS: Numeric = 60.0;
pub const SECONDS_TO_MINUTES: Numeric = 1.0 / MINUTES_TO_SECONDS;

pub const HOURS_TO_SECONDS: Numeric = HOURS_TO_MINUTES * MINUTES_TO_SECONDS;
pub const SECONDS_TO_HOURS: Numeric = 1.0 / HOURS_TO_SECONDS;

#[derive(Debug, Copy, Clone)]
pub enum Time {
    Hours(Numeric),
    Minutes(Numeric),
    Seconds(Numeric),
}
impl From<Time> for Numeric {
    fn from(u: Time) -> Numeric {
        match u {
            Hours(u) => u,
            Minutes(u) => u,
            Seconds(u) => u,
        }
    }
}
impl Time {
    pub fn to_num(self) -> Numeric {
        Numeric::from(self)
    }
    pub fn to_hours(self) -> Self {
        match self {
            u @ Hours(_) => u,
            Minutes(u) => Hours(u * MINUTES_TO_HOURS),
            Seconds(u) => Hours(u * SECONDS_TO_HOURS),
        }
    }
    pub fn to_minutes(self) -> Self {
        match self {
            u @ Minutes(_) => u,
            Hours(u) => Minutes(u * HOURS_TO_MINUTES),
            Seconds(u) => Minutes(u * SECONDS_TO_MINUTES),
        }
    }
    pub fn to_seconds(self) -> Self {
        match self {
            u @ Seconds(_) => u,
            Hours(u) => Seconds(u * HOURS_TO_SECONDS),
            Minutes(u) => Seconds(u * MINUTES_TO_SECONDS),
        }
    }
}
