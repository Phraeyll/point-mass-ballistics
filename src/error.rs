use crate::{
    Numeric,
    units::{Angle, ThermodynamicTemperature, Time},
};

use std::{error, fmt, result};

pub type Result<T, E = Error> = result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    PositiveExpected {
        value: Numeric,
    },
    NegativeExpected {
        value: Numeric,
    },
    NumericOutOfRange {
        value: Numeric,
        min: Numeric,
        max: Numeric,
    },
    AngleOutOfRange {
        value: Angle,
        min: Angle,
        max: Angle,
    },
    ThermodynamicTemperatureOutOfRange {
        value: ThermodynamicTemperature,
        min: ThermodynamicTemperature,
        max: ThermodynamicTemperature,
    },
    TimeOutOfRange {
        value: Time,
        min: Time,
        max: Time,
    },
    ZeroAngleOutOfRange {
        count: u64,
        pitch: Angle,
        yaw: Angle,
    },
    ZeroAngleNotChanging {
        count: u64,
        pitch: Angle,
        yaw: Angle,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl error::Error for Error {}
