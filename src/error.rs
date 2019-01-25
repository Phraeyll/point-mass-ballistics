use crate::util::conversions::*;
use crate::util::Numeric;

use std::error::Error as StdError;
use std::fmt;
use std::fmt::Display as StdDisplay;
use std::result;
use std::str;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error(Box::new(kind))
    }
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    VelocityLookup(Numeric),
    PositiveExpected(Numeric),
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

impl StdDisplay for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::VelocityLookup(ref err) => write!(f, "Velocity Lookup Error: {}", err),
            ErrorKind::PositiveExpected(ref err) => write!(f, "Positive Expected Error: {}", err),
            ErrorKind::OutOfRange { ref min, ref max } => write!(
                f,
                "Within Range Expected Error => min: {:#?} - {:#?}",
                min, max
            ),
            ErrorKind::AngleRange { count, pitch, yaw } => write!(
                f,
                "{}: Outside Valid Range Error => pitch: {:#?}, yaw: {:#?}",
                count, pitch, yaw
            ),
            ErrorKind::TerminalVelocity { count, pitch, yaw } => write!(
                f,
                "{}: Terminal Velocity Error => pitch: {:#?}, yaw: {:#?}",
                count, pitch, yaw
            ),
            ErrorKind::AngleNotChanging { count, pitch, yaw } => write!(
                f,
                "{}: Angle Not Changing Error => pitch: {:#?}, yaw: {:#?}",
                count, pitch, yaw
            ),
        }
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        match *self.0 {
            ErrorKind::VelocityLookup(_) => "Velocity out of range",
            ErrorKind::PositiveExpected(..) => "Number needs to be positive greater than 0",
            ErrorKind::OutOfRange { .. } => "Numer needs to be within range",
            ErrorKind::AngleRange { .. } => "Angle out of range",
            ErrorKind::TerminalVelocity { .. } => "Terminal velocity reached",
            ErrorKind::AngleNotChanging { .. } => "Angle not changing curing iteration",
        }
    }
}
