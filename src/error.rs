use crate::util::{Angle, Numeric};

use std::{error::Error as StdError, fmt, fmt::Debug, result::Result as StdResult};

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug)]
pub enum Error {
    VelocityLookup(Numeric),
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
        match self {
            Self::VelocityLookup(ref err) => write!(f, "Velocity Lookup Error: {:?}", err),
            Self::PositiveExpected(ref err) => write!(f, "Positive Expected Error: {:?}", err),
            Self::NegativeExpected(ref err) => write!(f, "Negative Expected Error: {:?}", err),
            Self::OutOfRange { ref min, ref max } => write!(
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

impl StdError for Error {}
