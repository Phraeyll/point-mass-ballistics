use self::Length::*;
use crate::util::Numeric;

pub(super) const MILES_TO_YARDS: Numeric = 1_760.0;
pub(super) const YARDS_TO_MILES: Numeric = 1.0 / MILES_TO_YARDS;

pub(super) const YARDS_TO_FEET: Numeric = 3.0;
pub(super) const FEET_TO_YARDS: Numeric = 1.0 / YARDS_TO_FEET;

pub(super) const FEET_TO_INCHES: Numeric = 12.0;
pub(super) const INCHES_TO_FEET: Numeric = 1.0 / FEET_TO_INCHES;

pub(super) const FEET_TO_METERS: Numeric = 0.304_8;
pub(super) const METERS_TO_FEET: Numeric = 1.0 / FEET_TO_METERS;

#[derive(Debug, Copy, Clone)]
pub enum Length {
    Meters(Numeric),
    Miles(Numeric),
    Yards(Numeric),
    Feet(Numeric),
    Inches(Numeric),
}
impl From<Length> for Numeric {
    fn from(u: Length) -> Numeric {
        match u {
            Meters(u) => u,
            Miles(u) => u,
            Yards(u) => u,
            Feet(u) => u,
            Inches(u) => u,
        }
    }
}
impl Length {
    pub fn to_num(self) -> Numeric {
        From::from(self)
    }
    pub fn to_meters(self) -> Self {
        match self {
            u @ Meters(_) => u,
            u @ Miles(_) => u.to_feet().to_meters(),
            u @ Yards(_) => u.to_feet().to_meters(),
            u @ Inches(_) => u.to_feet().to_meters(),
            Feet(u) => Meters(u * FEET_TO_METERS),
        }
    }
    pub fn to_inches(self) -> Self {
        match self {
            u @ Inches(_) => u,
            u @ Meters(_) => u.to_feet().to_inches(),
            u @ Miles(_) => u.to_feet().to_inches(),
            u @ Yards(_) => u.to_feet().to_inches(),
            Feet(u) => Inches(u * FEET_TO_INCHES),
        }
    }
    pub fn to_yards(self) -> Self {
        match self {
            u @ Yards(_) => u,
            u @ Meters(_) => u.to_feet().to_yards(),
            u @ Inches(_) => u.to_feet().to_yards(),
            Miles(u) => Yards(u * MILES_TO_YARDS),
            Feet(u) => Yards(u * FEET_TO_YARDS),
        }
    }
    pub fn to_miles(self) -> Self {
        match self {
            u @ Miles(_) => u,
            u @ Meters(_) => u.to_feet().to_miles(),
            u @ Feet(_) => u.to_feet().to_miles(),
            u @ Inches(_) => u.to_feet().to_miles(),
            Yards(u) => Miles(u * YARDS_TO_MILES),
        }
    }
    pub fn to_feet(self) -> Self {
        match self {
            u @ Feet(_) => u,
            u @ Miles(_) => u.to_yards().to_feet(),
            Meters(u) => Feet(u * METERS_TO_FEET),
            Yards(u) => Feet(u * YARDS_TO_FEET),
            Inches(u) => Feet(u * INCHES_TO_FEET),
        }
    }
}
