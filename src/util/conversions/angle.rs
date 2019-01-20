use crate::util::Numeric;
use Angle::*;

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
