use crate::util::Numeric;
use Length::*;

use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

pub const MILES_TO_YARDS: Numeric = 1_760.0;
pub const YARDS_TO_MILES: Numeric = 1.0 / MILES_TO_YARDS;

pub const YARDS_TO_FEET: Numeric = 3.0;
pub const FEET_TO_YARDS: Numeric = 1.0 / YARDS_TO_FEET;

pub const FEET_TO_INCHES: Numeric = 12.0;
pub const INCHES_TO_FEET: Numeric = 1.0 / FEET_TO_INCHES;

pub const FEET_TO_METERS: Numeric = 0.304_8;
pub const METERS_TO_FEET: Numeric = 1.0 / FEET_TO_METERS;

pub const MILES_TO_FEET: Numeric = MILES_TO_YARDS * YARDS_TO_FEET;
pub const FEET_TO_MILES: Numeric = 1.0 / MILES_TO_FEET;

pub const YARDS_TO_INCHES: Numeric = YARDS_TO_FEET * FEET_TO_INCHES;
pub const INCHES_TO_YARDS: Numeric = 1.0 / YARDS_TO_INCHES;

pub const MILES_TO_INCHES: Numeric = MILES_TO_YARDS * YARDS_TO_INCHES;
pub const INCHES_TO_MILES: Numeric = 1.0 / MILES_TO_INCHES;

pub const MILES_TO_METERS: Numeric = MILES_TO_FEET * FEET_TO_METERS;
pub const METERS_TO_MILES: Numeric = 1.0 / MILES_TO_METERS;

pub const YARDS_TO_METERS: Numeric = YARDS_TO_FEET * FEET_TO_METERS;
pub const METERS_TO_YARDS: Numeric = 1.0 / YARDS_TO_METERS;

pub const INCHES_TO_METERS: Numeric = INCHES_TO_FEET * FEET_TO_METERS;
pub const METERS_TO_INCHES: Numeric = 1.0 / INCHES_TO_METERS;

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
        Numeric::from(self)
    }
    pub fn to_meters(self) -> Self {
        match self {
            u @ Meters(_) => u,
            Miles(u) => Meters(u * MILES_TO_METERS),
            Yards(u) => Meters(u * YARDS_TO_METERS),
            Feet(u) => Meters(u * FEET_TO_METERS),
            Inches(u) => Meters(u * INCHES_TO_METERS),
        }
    }
    pub fn to_inches(self) -> Self {
        match self {
            u @ Inches(_) => u,
            Meters(u) => Inches(u * METERS_TO_INCHES),
            Yards(u) => Inches(u * YARDS_TO_INCHES),
            Feet(u) => Inches(u * FEET_TO_INCHES),
            Miles(u) => Inches(u * MILES_TO_INCHES),
        }
    }
    pub fn to_yards(self) -> Self {
        match self {
            u @ Yards(_) => u,
            Meters(u) => Yards(u * METERS_TO_YARDS),
            Feet(u) => Yards(u * FEET_TO_YARDS),
            Miles(u) => Yards(u * MILES_TO_YARDS),
            Inches(u) => Yards(u * INCHES_TO_YARDS),
        }
    }
    pub fn to_miles(self) -> Self {
        match self {
            u @ Miles(_) => u,
            Meters(u) => Miles(u * METERS_TO_MILES),
            Feet(u) => Miles(u * FEET_TO_MILES),
            Yards(u) => Miles(u * YARDS_TO_MILES),
            Inches(u) => Miles(u * INCHES_TO_MILES),
        }
    }
    pub fn to_feet(self) -> Self {
        match self {
            u @ Feet(_) => u,
            Meters(u) => Feet(u * METERS_TO_FEET),
            Yards(u) => Feet(u * YARDS_TO_FEET),
            Miles(u) => Feet(u * MILES_TO_FEET),
            Inches(u) => Feet(u * INCHES_TO_FEET),
        }
    }
}

impl PartialEq for Length {
    fn eq(&self, other: &Length) -> bool {
        match *self {
            Meters(u) => u == other.to_meters().to_num(),
            Miles(u) => u == other.to_miles().to_num(),
            Yards(u) => u == other.to_yards().to_num(),
            Feet(u) => u == other.to_feet().to_num(),
            Inches(u) => u == other.to_inches().to_num(),
        }
    }
}
impl Neg for Length {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Meters(u) => Meters(-u),
            Miles(u) => Miles(-u),
            Yards(u) => Yards(-u),
            Feet(u) => Feet(-u),
            Inches(u) => Inches(-u),
        }
    }
}

impl Add for Length {
    type Output = Self;
    fn add(self, other: Length) -> Self::Output {
        match self {
            Meters(u) => Meters(u + other.to_meters().to_num()),
            Miles(u) => Miles(u + other.to_miles().to_num()),
            Yards(u) => Yards(u + other.to_yards().to_num()),
            Feet(u) => Feet(u + other.to_feet().to_num()),
            Inches(u) => Inches(u + other.to_inches().to_num()),
        }
    }
}
impl AddAssign for Length {
    fn add_assign(&mut self, other: Length) {
        *self = match *self {
            u @ Meters(_) => u + other,
            u @ Miles(_) => u + other,
            u @ Yards(_) => u + other,
            u @ Feet(_) => u + other,
            u @ Inches(_) => u + other,
        };
    }
}
impl Sub for Length {
    type Output = Self;
    fn sub(self, other: Length) -> Self::Output {
        match self {
            Meters(u) => Meters(u - other.to_meters().to_num()),
            Miles(u) => Miles(u - other.to_miles().to_num()),
            Yards(u) => Yards(u - other.to_yards().to_num()),
            Feet(u) => Feet(u - other.to_feet().to_num()),
            Inches(u) => Inches(u - other.to_inches().to_num()),
        }
    }
}
impl SubAssign for Length {
    fn sub_assign(&mut self, other: Length) {
        *self = match *self {
            u @ Meters(_) => u - other,
            u @ Miles(_) => u - other,
            u @ Yards(_) => u - other,
            u @ Feet(_) => u - other,
            u @ Inches(_) => u - other,
        };
    }
}
