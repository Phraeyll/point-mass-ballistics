use self::ErrorKind::*;
use crate::util::*;

use std::{error::Error as StdError, fmt, fmt::Display as StdDisplay, result, str};

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
    BcKindNull,
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
            BcKindNull => write!(f, "Bc need to be set before inititializing simulatin"),
            VelocityLookup(ref err) => write!(f, "Velocity Lookup Error: {}", err),
            PositiveExpected(ref err) => write!(f, "Positive Expected Error: {}", err),
            OutOfRange { ref min, ref max } => write!(
                f,
                "Within Range Expected Error => min: {:#?} - {:#?}",
                min, max
            ),
            AngleRange { count, pitch, yaw } => write!(
                f,
                "{}: Outside Valid Range Error => pitch: {:#?}, yaw: {:#?}",
                count, pitch, yaw
            ),
            TerminalVelocity { count, pitch, yaw } => write!(
                f,
                "{}: Terminal Velocity Error => pitch: {:#?}, yaw: {:#?}",
                count, pitch, yaw
            ),
            AngleNotChanging { count, pitch, yaw } => write!(
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
            BcKindNull => "BcKind is null",
            VelocityLookup(_) => "Velocity out of range",
            PositiveExpected(..) => "Number needs to be positive greater than 0",
            OutOfRange { .. } => "Numer needs to be within range",
            AngleRange { .. } => "Angle out of range",
            TerminalVelocity { .. } => "Terminal velocity reached",
            AngleNotChanging { .. } => "Angle not changing curing iteration",
        }
    }
}
