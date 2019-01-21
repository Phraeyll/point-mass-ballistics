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
    OutOfRange(Numeric, Numeric),
    AngleRange(u64, Numeric),
    TerminalVelocity(u64, Numeric),
    AngleNotChanging(u64, Numeric),
}

impl StdDisplay for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::VelocityLookup(ref err) => {
                write!(formatter, "Velocity Lookup Error: {}", err)
            }
            ErrorKind::PositiveExpected(ref err) => {
                write!(formatter, "Positive Expected Error: {}", err)
            }
            ErrorKind::OutOfRange(ref start, ref end) => write!(
                formatter,
                "Within Range Expected Error: {} - {}",
                start, end
            ),
            ErrorKind::AngleRange(count, angle) => {
                write!(formatter, "{}: Outside Valid Range Error: {}", count, angle)
            }
            ErrorKind::TerminalVelocity(count, angle) => {
                write!(formatter, "{}: Terminal Velocity Error: {}", count, angle)
            }
            ErrorKind::AngleNotChanging(count, angle) => {
                write!(formatter, "{}: Angle Not Changing Error: {}", count, angle)
            }
        }
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        match *self.0 {
            ErrorKind::VelocityLookup(_) => "Velocity out of range",
            ErrorKind::PositiveExpected(..) => "Number needs to be positive greater than 0",
            ErrorKind::OutOfRange(..) => "Numer needs to be within range",
            ErrorKind::AngleRange(..) => "Angle out of range",
            ErrorKind::TerminalVelocity(..) => "Terminal velocity reached",
            ErrorKind::AngleNotChanging(..) => "Angle not changing curing iteration",
        }
    }
}
