use crate::{
    model::core::{SimulationBuilder, WindAdjuster},
    util::*,
};

#[derive(Debug)]
pub struct Wind {
    pub(crate) yaw: Angle,         // Wind Angle (degrees)
    pub(crate) pitch: Angle,       // Wind Pitch (degrees)
    pub(crate) roll: Angle,        // Doesn't make sense, just here for consistency
    pub(crate) velocity: Velocity, // Wind Velocity (miles/hour)
}
#[derive(Debug)]
pub struct WindBuilder {
    pub yaw: Angle,         // Wind Angle (degrees)
    pub pitch: Angle,       // Wind Pitch (degrees)
    pub roll: Angle,        // Doesn't make sense, just here for consistency
    pub velocity: Velocity, // Wind Velocity (miles/hour)
}
impl From<WindBuilder> for Wind {
    fn from(other: WindBuilder) -> Self {
        Self {
            yaw: other.yaw,
            pitch: other.pitch,
            roll: other.roll,
            velocity: other.velocity,
        }
    }
}
impl From<Wind> for WindBuilder {
    fn from(other: Wind) -> Self {
        Self {
            yaw: other.yaw,
            pitch: other.pitch,
            roll: other.roll,
            velocity: other.velocity,
        }
    }
}
impl Default for WindBuilder {
    fn default() -> Self {
        Self {
            yaw: Angle::Degrees(0.0),
            pitch: Angle::Degrees(0.0),
            roll: Angle::Degrees(0.0),
            velocity: Velocity::Mph(0.0),
        }
    }
}
impl WindAdjuster for SimulationBuilder {
    fn set_wind_speed(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.wind.velocity = Velocity::Mph(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn set_wind_angle(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-360.0, 360.0);
        if value >= min && value <= max {
            self.wind.yaw = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
        }
    }
}
