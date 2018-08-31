use super::consts::*;
use self::Time::*;

#[derive(Copy, Clone)]
pub enum Time {
    Hours(f64),
    Minutes(f64),
    Seconds(f64),
}
impl From<Time> for f64 {
    fn from(u: Time) -> f64 {
        match u {
            Hours(u) => u,
            Minutes(u) => u,
            Seconds(u) => u,
        }
    }
}
impl self::Time {
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
