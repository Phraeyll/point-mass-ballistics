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
impl WindAdjuster for SimulationBuilder {
    fn set_wind_speed(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.wind.velocity = Velocity::Mph(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn set_wind_angle(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-360.0, 360.0);
        if value >= min && value <= max {
            self.builder.wind.yaw = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
        }
    }
}
