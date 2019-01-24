use crate::model::core::{AtmosphereAdjuster, SimulationBuilder};
use crate::util::*;

const UNIVERSAL_GAS: Numeric = 8.314_459_8; // Universal gas constant (J/K*mol)
const MOLAR_DRY: Numeric = 0.028_964_4; // Molar mass of dry air (kg/mol)
const MOLAR_VAPOR: Numeric = 0.018_016; // Molar mass of water vapor (kg/mol)
const ADIABATIC_INDEX_AIR: Numeric = 1.4; // Adiabatic index of air, mostly diatomic gas

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
impl From<AtmosphereBuilder> for Atmosphere{
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
}

impl Atmosphere {
    // Density of air, using pressure, humidity, and temperature
    pub(crate) fn rho(&self) -> Numeric {
        ((self.pd() * MOLAR_DRY) + (self.pv() * MOLAR_VAPOR)) / (UNIVERSAL_GAS * self.kelvin())
    }
    // Speed of sound at given air density and pressure
    pub(crate) fn speed_of_sound(&self) -> Numeric {
        (ADIABATIC_INDEX_AIR * (self.pa() / self.rho())).sqrt()
    }
    // Pressure of water vapor, Arden Buck equation
    fn pv(&self) -> Numeric {
        self.humidity
            * 611.21
            * ((18.678 - (self.celsius() / 234.5)) * (self.celsius() / (257.14 + self.celsius())))
                .exp()
    }
    // Pressure of dry air
    fn pd(&self) -> Numeric {
        self.pa() - self.pv()
    }
    // Total air pressure in pascals
    fn pa(&self) -> Numeric {
        self.pressure.to_pascals().to_num()
    }
    // Temperature in celsius
    fn celsius(&self) -> Numeric {
        self.temperature.to_celsius().to_num()
    }
    // Temperature in kelvin
    fn kelvin(&self) -> Numeric {
        self.temperature.to_kelvin().to_num()
    }
}
