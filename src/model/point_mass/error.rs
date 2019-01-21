use super::builder;
use super::zero;
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
    Zeroing(zero::Error),
    VelocityLookup(Numeric),
    Builder(builder::Error),
}

impl StdDisplay for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Zeroing(ref err) => write!(formatter, "Zeroing Error: {}", err),
            ErrorKind::VelocityLookup(ref err) => {
                write!(formatter, "Velocity Lookup Error: {}", err)
            }
            ErrorKind::Builder(ref err) => write!(formatter, "Builder Error: {}", err),
        }
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        match *self.0 {
            ErrorKind::Zeroing(ref err, ..) => err.description(),
            ErrorKind::VelocityLookup(_) => "Velocity out of range",
            ErrorKind::Builder(_) => "Invalid inputs",
        }
    }
}

impl From<zero::Error> for Error {
    fn from(err: zero::Error) -> Self {
        Error::new(ErrorKind::Zeroing(err))
    }
}
impl From<builder::Error> for Error {
    fn from(err: builder::Error) -> Self {
        Error::new(ErrorKind::Builder(err))
    }
}
