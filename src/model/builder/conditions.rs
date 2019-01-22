use crate::util::*;
use crate::model::core::{Conditions, Atmosphere, Wind, Other};

impl Default for Conditions {
    fn default() -> Self {
        Self {
            wind: Wind::default(),
            atmosphere: Atmosphere::default(),
            other: Other::default(),
        }
    }
}
impl Default for Wind {
    fn default() -> Self {
        Self {
            velocity: Velocity::Mph(0.0),
            yaw: Angle::Radians(0.0),
        }
    }
}
impl Default for Atmosphere {
    fn default() -> Self {
        Self {
            temperature: Temperature::F(68.0),
            pressure: Pressure::Inhg(29.92),
            humidity: 0.0,
        }
    }
}
impl Default for Other {
    fn default() -> Self {
        Self {
            line_of_sight: Angle::Radians(0.0),
            azimuth: Angle::Radians(0.0),
            lattitude: Angle::Radians(0.0),
            gravity: Acceleration::Mps2(crate::model::core::GRAVITY),
        }
    }
}
pub trait MutateConditions {
    fn new() -> Self;
    fn set_temperature(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_pressure(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_humidity(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_wind_speed(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_wind_angle(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_shot_angle(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_lattitude(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_bearing(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_gravity(self, value: Numeric) -> Self;
}
impl MutateConditions for Conditions {
    fn new() -> Self {
        Self::default()
    }
    fn set_temperature(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-112.0, 122.0);
        if value >= min && value <= max {
            self.atmosphere.temperature = Temperature::F(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn set_pressure(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.atmosphere.pressure = Pressure::Inhg(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn set_humidity(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (0.0, 1.0);
        if value >= min && value <= max {
            self.atmosphere.humidity = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
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
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn set_shot_angle(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-90.0, 90.0);
        if value >= min && value <= max {
            self.other.line_of_sight = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn set_lattitude(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-90.0, 90.0);
        if value >= min && value <= max {
            self.other.lattitude = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn set_bearing(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-360.0, 360.0);
        if value >= min && value <= max {
            self.other.azimuth = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn set_gravity(mut self, value: Numeric) -> Self {
        self.other.gravity = Acceleration::Fps2(value);
        self
    }
}
