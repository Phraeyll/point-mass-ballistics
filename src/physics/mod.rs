pub use self::VelocityKind::*;

mod consts;

use self::consts::*;
use conversions::*;

use std::f64;
use std::f64::consts::E;

pub enum VelocityKind {
    Projectile(f64),
    Wind(f64),
}

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
pub fn velocity_tuple(vk: VelocityKind, deg: f64) -> (f64, f64, f64) {
    let angle = deg.to_radians();
    match vk {
        VelocityKind::Projectile(vel) => {
            let velocity = f64::from(Velocity::Fps(vel).to_mps());
            (velocity * angle.cos(), velocity * angle.sin(), 0.0)
        }
        VelocityKind::Wind(vel) => {
            let velocity = f64::from(Velocity::Mph(vel).to_mps());
            (velocity * angle.cos(), 0.0, velocity * angle.sin())
        }
    }
}
pub fn mass(weight_grains: f64) -> f64 {
    f64::from(WeightMass::Grains(weight_grains).to_kgs())
}
pub fn radius(caliber: f64) -> f64 {
    f64::from(Length::Inches(caliber).to_meters()) / 2.0
}
pub fn form_factor(weight_grains: f64, caliber: f64, bc: f64) -> f64 {
    f64::from(WeightMass::Grains(weight_grains).to_lbs()) / (caliber.powf(2.0) * bc)
}
pub fn gravity() -> (f64, f64, f64) {
    (0.0, GRAVITY, 0.0)
}
