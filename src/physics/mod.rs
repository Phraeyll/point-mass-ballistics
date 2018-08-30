mod consts;

use self::consts::*;
use conversions::*;

use std::f64;
use std::f64::consts::E;

pub fn air_density(temp: f64, humidity: f64, pressure: f64) -> f64 {
    let celsius = f64::from(Temperature::F(temp).to_celsius());
    let kelvin = f64::from(Temperature::F(temp).to_kelvin());
    let pa = f64::from(Pressure::Inhg(pressure).to_pascals());
    let pv =
        humidity * 611.21 * E.powf((18.678 - (celsius / 234.5)) * (celsius / (257.14 + celsius)));
    let pd = pa - pv;
    ((pd * MOLAR_DRY) + (pv * MOLAR_VAPOR)) / (UNIVERSAL_GAS * kelvin)
}

pub fn speed_sound(rho: f64, pressure: f64) -> f64 {
    let pa = f64::from(Pressure::Inhg(pressure).to_pascals());
    (1.4 * (pa / rho)).sqrt()
}

pub fn gravity() -> f64 {
    GRAVITY
}
