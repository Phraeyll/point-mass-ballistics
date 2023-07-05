use crate::{units::Angle, Numeric};

use std::{error, fmt, result};

pub type Result<T, E = Error> = result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    Mach(Numeric),
    PositiveExpected(Numeric),
    NegativeExpected(Numeric),
    OutOfRange {
        min: Numeric,
        max: Numeric,
    },
    AngleRange {
        count: u64,
        pitch: Angle,
        yaw: Angle,
    },
    TerminalVelocity {
        count: u64,
        pitch: Angle,
        yaw: Angle,
    },
    AngleNotChanging {
        count: u64,
        pitch: Angle,
        yaw: Angle,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Mach(err) => write!(f, "Mach Lookup Too High: {:?}", err),
            Self::PositiveExpected(err) => write!(f, "Positive Expected Error: {:?}", err),
            Self::NegativeExpected(err) => write!(f, "Negative Expected Error: {:?}", err),
            Self::OutOfRange { min, max } => write!(
                f,
                "Within Range Expected Error => min: {:#?} - {:#?}",
                min, max
            ),
            Self::AngleRange { count, pitch, yaw } => write!(
                f,
                "{}: Outside Valid Range Error => pitch: {:#?}, yaw: {:#?}",
                count, pitch, yaw
            ),
            Self::TerminalVelocity { count, pitch, yaw } => write!(
                f,
                "{}: Terminal Velocity Error => pitch: {:#?}, yaw: {:#?}",
                count, pitch, yaw
            ),
            Self::AngleNotChanging { count, pitch, yaw } => write!(
                f,
                "{}: Angle Not Changing Error => pitch: {:#?}, yaw: {:#?}",
                count, pitch, yaw
            ),
        }
    }
}

impl error::Error for Error {}
