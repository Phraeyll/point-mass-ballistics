use ordered_float::OrderedFloat;

use self::Angle::*;
use crate::util::Numeric;

use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

pub const DEGREES_TO_MINUTES: Numeric = 60.0;
pub const MINUTES_TO_DEGREES: Numeric = 1.0 / DEGREES_TO_MINUTES;
pub const RADIANS_TO_MILIRADIANS: Numeric = 1000.0;
pub const MILIRADIANS_TO_RADIANS: Numeric = 1.0 / RADIANS_TO_MILIRADIANS;

#[derive(Debug, Copy, Clone)]
pub enum Angle {
    Degrees(Numeric),
    Minutes(Numeric),
    Radians(Numeric),
    Miliradians(Numeric),
}
impl From<Angle> for Numeric {
    fn from(u: Angle) -> Numeric {
        match u {
            Degrees(u) => u,
            Minutes(u) => u,
            Radians(u) => u,
            Miliradians(u) => u,
        }
    }
}
impl Angle {
    pub fn to_num(self) -> Numeric {
        Numeric::from(self)
    }
    pub fn to_degrees(self) -> Self {
        match self {
            u @ Degrees(_) => u,
            Minutes(u) => Degrees(u * MINUTES_TO_DEGREES),
            Radians(u) => Degrees(u.to_radians()),
            Miliradians(u) => Degrees((u * MILIRADIANS_TO_RADIANS).to_degrees()),
        }
    }
    pub fn to_minutes(self) -> Self {
        match self {
            u @ Minutes(_) => u,
            Degrees(u) => Minutes(u * DEGREES_TO_MINUTES),
            Radians(u) => Minutes(u.to_degrees() * DEGREES_TO_MINUTES),
            Miliradians(u) => {
                Minutes((u * MILIRADIANS_TO_RADIANS).to_degrees() * DEGREES_TO_MINUTES)
            }
        }
    }
    pub fn to_radians(self) -> Self {
        match self {
            u @ Radians(_) => u,
            Minutes(u) => Radians((u * MINUTES_TO_DEGREES).to_radians()),
            Degrees(u) => Radians(u.to_radians()),
            Miliradians(u) => Radians(u * MILIRADIANS_TO_RADIANS),
        }
    }
    pub fn to_miliradians(self) -> Self {
        match self {
            u @ Miliradians(_) => u,
            Minutes(u) => {
                Miliradians((u * MINUTES_TO_DEGREES).to_radians() * RADIANS_TO_MILIRADIANS)
            }
            Radians(u) => Miliradians(u * RADIANS_TO_MILIRADIANS),
            Degrees(u) => Miliradians(u.to_radians() * RADIANS_TO_MILIRADIANS),
        }
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Angle) -> bool {
        match *self {
            Degrees(u) => u == other.to_degrees().to_num(),
            Minutes(u) => u == other.to_minutes().to_num(),
            Radians(u) => u == other.to_radians().to_num(),
            Miliradians(u) => u == other.to_miliradians().to_num(),
        }
    }
}
impl Eq for Angle {}
impl Ord for Angle {
    fn cmp(&self, other: &Angle) -> Ordering {
        match *self {
            Degrees(u) => OrderedFloat(u).cmp(&OrderedFloat(other.to_degrees().to_num())),
            Minutes(u) => OrderedFloat(u).cmp(&OrderedFloat(other.to_minutes().to_num())),
            Radians(u) => OrderedFloat(u).cmp(&OrderedFloat(other.to_radians().to_num())),
            Miliradians(u) => OrderedFloat(u).cmp(&OrderedFloat(other.to_miliradians().to_num())),
        }
    }
}
impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Angle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Neg for Angle {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Degrees(u) => Degrees(-u),
            Minutes(u) => Minutes(-u),
            Radians(u) => Radians(-u),
            Miliradians(u) => Miliradians(-u),
        }
    }
}

impl Add for Angle {
    type Output = Self;
    fn add(self, other: Angle) -> Self::Output {
        match self {
            Degrees(u) => Degrees(u + other.to_degrees().to_num()),
            Minutes(u) => Minutes(u + other.to_minutes().to_num()),
            Radians(u) => Radians(u + other.to_radians().to_num()),
            Miliradians(u) => Miliradians(u + other.to_miliradians().to_num()),
        }
    }
}
impl AddAssign for Angle {
    fn add_assign(&mut self, other: Angle) {
        *self = match *self {
            u @ Degrees(_) => u + other,
            u @ Minutes(_) => u + other,
            u @ Radians(_) => u + other,
            u @ Miliradians(_) => u + other,
        };
    }
}
impl Sub for Angle {
    type Output = Self;
    fn sub(self, other: Angle) -> Self::Output {
        match self {
            Degrees(u) => Degrees(u - other.to_degrees().to_num()),
            Minutes(u) => Minutes(u - other.to_minutes().to_num()),
            Radians(u) => Radians(u - other.to_radians().to_num()),
            Miliradians(u) => Miliradians(u - other.to_miliradians().to_num()),
        }
    }
}
impl SubAssign for Angle {
    fn sub_assign(&mut self, other: Angle) {
        *self = match *self {
            u @ Degrees(_) => u - other,
            u @ Minutes(_) => u - other,
            u @ Radians(_) => u - other,
            u @ Miliradians(_) => u - other,
        };
    }
}
