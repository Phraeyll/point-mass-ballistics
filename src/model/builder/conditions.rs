use crate::util::*;

#[derive(Debug)]
pub struct Conditions {
    pub(crate) wind: Wind,
    pub(crate) atmosphere: Atmosphere,
    pub(crate) other: Other,
}
impl Default for Conditions {
    fn default() -> Self {
        Self {
            wind: Wind::default(),
            atmosphere: Atmosphere::default(),
            other: Other::default(),
        }
    }
}

#[derive(Debug)]
pub struct Wind {
    pub(crate) velocity: Velocity, // Wind Velocity (miles/hour)
    pub(crate) yaw: Angle,         // Wind Angle (degrees)
}
impl Default for Wind {
    fn default() -> Self {
        Self {
            velocity: Velocity::Mph(0.0),
            yaw: Angle::Radians(0.0),
        }
    }
}

#[derive(Debug)]
pub struct Atmosphere {
    pub(crate) temperature: Temperature, // Temperature (F)
    pub(crate) pressure: Pressure,       // Pressure (InHg)
    pub(crate) humidity: Numeric,        // Humidity (0-1)
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

#[derive(Debug)]
pub struct Other {
    pub(crate) line_of_sight: Angle,  // Line of Sight angle (degrees)
    pub(crate) azimuth: Angle, // Bearing (0 North, 90 East) (degrees) (Coriolis/Eotvos Effect)
    pub(crate) lattitude: Angle, // Lattitude (Coriolis/Eotvos Effect)
    pub(crate) gravity: Acceleration, // Gravity (m/s^2)
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

pub trait ConditionsBuilder {
    fn new() -> Self;
    fn with_temperature(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_pressure(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_humidity(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_wind_speed(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_wind_angle(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_shot_angle(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_lattitude(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_bearing(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_gravity(self, value: Numeric) -> Self;
}
impl ConditionsBuilder for Conditions {
    fn new() -> Self {
        Self::default()
    }
    fn with_temperature(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-112.0, 122.0);
        if value >= min && value <= max {
            self.atmosphere.temperature = Temperature::F(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn with_pressure(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.atmosphere.pressure = Pressure::Inhg(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn with_humidity(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (0.0, 1.0);
        if value >= min && value <= max {
            self.atmosphere.humidity = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn with_wind_speed(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.wind.velocity = Velocity::Mph(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn with_wind_angle(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-360.0, 360.0);
        if value >= min && value <= max {
            self.wind.yaw = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn with_shot_angle(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-90.0, 90.0);
        if value >= min && value <= max {
            self.other.line_of_sight = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn with_lattitude(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-90.0, 90.0);
        if value >= min && value <= max {
            self.other.lattitude = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn with_bearing(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-360.0, 360.0);
        if value >= min && value <= max {
            self.other.azimuth = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn with_gravity(mut self, value: Numeric) -> Self {
        self.other.gravity = Acceleration::Fps2(value);
        self
    }
}
