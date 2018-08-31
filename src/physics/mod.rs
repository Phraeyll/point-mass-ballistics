use conversions::*;
use self::consts::*;

use std::f64;
use std::f64::consts::E;

mod consts;

pub fn air_density(temperature: Temperature, pressure: Pressure, humidity: f64) -> f64 {
    let celsius = f64::from(temperature.to_celsius());
    let kelvin = f64::from(temperature.to_kelvin());
    let pa = f64::from(pressure.to_pascals());
    let pv =
        humidity * 611.21 * E.powf((18.678 - (celsius / 234.5)) * (celsius / (257.14 + celsius)));
    let pd = pa - pv;
    ((pd * MOLAR_DRY) + (pv * MOLAR_VAPOR)) / (UNIVERSAL_GAS * kelvin)
}

pub fn speed_sound(rho: f64, pressure: Pressure) -> f64 {
    let pa = f64::from(pressure.to_pascals());
    (1.4 * (pa / rho)).sqrt()
}

pub fn gravity() -> Acceleration {
    GRAVITY
}
