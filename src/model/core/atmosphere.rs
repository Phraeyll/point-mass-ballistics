use crate::{
    model::core::{AtmosphereAdjuster, SimulationBuilder},
    util::*,
};

#[derive(Debug)]
pub struct Atmosphere {
    pub(crate) temperature: Temperature, // Temperature (F)
    pub(crate) pressure: Pressure,       // Pressure (InHg)
    pub(crate) humidity: Numeric,        // Humidity (0-1)
}
#[derive(Debug)]
pub struct AtmosphereBuilder {
    pub temperature: Temperature, // Temperature (F)
    pub pressure: Pressure,       // Pressure (InHg)
    pub humidity: Numeric,        // Humidity (0-1)
}
impl From<AtmosphereBuilder> for Atmosphere {
    fn from(other: AtmosphereBuilder) -> Self {
        Self {
            temperature: other.temperature,
            pressure: other.pressure,
            humidity: other.humidity,
        }
    }
}
impl From<Atmosphere> for AtmosphereBuilder {
    fn from(other: Atmosphere) -> Self {
        Self {
            temperature: other.temperature,
            pressure: other.pressure,
            humidity: other.humidity,
        }
    }
}
impl Default for AtmosphereBuilder {
    fn default() -> Self {
        Self {
            temperature: Temperature::F(68.0),
            pressure: Pressure::Inhg(29.92),
            humidity: 0.0,
        }
    }
}
impl AtmosphereAdjuster for SimulationBuilder {
    fn set_temperature(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-112.0, 122.0);
        if value >= min && value <= max {
            self.atmosphere.temperature = Temperature::F(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
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
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
        }
    }
}
