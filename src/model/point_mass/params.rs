pub use super::dragtables::*;
use nalgebra::Vector3;

use crate::util::*;

use std::ops::Mul;

const GRAVITY: Numeric = -9.806_65; // Local gravity in m/s
const UNIVERSAL_GAS: Numeric = 8.314_459_8; // Universal gas constant (J/K*mol)
const MOLAR_DRY: Numeric = 0.028_964_4; // Molar mass of dry air (kg/mol)
const MOLAR_VAPOR: Numeric = 0.018_016; // Molar mass of water vapor (kg/mol)
const ADIABATIC_INDEX_AIR: Numeric = 1.4; // Adiabatic index of air, mostly diatomic gas

pub struct UnConditional {
    weight: WeightMass,                // Weight (grains)
    caliber: Length,                   // Caliber (inches)
    bc: BallisticCoefficient,          // Ballistic Coefficient
    pub drag_table: FloatMap<Numeric>, // Drag Function DragTable
    pub time_step: Time,               // Timestep for simulation (s)
    pub muzzle_velocity: Velocity,     // Initial velocity (ft/s)
    scope_height: Length,              // Scope Height (inches)
}
impl UnConditional {
    pub fn new(
        weight: Numeric,
        caliber: Numeric,
        bc: BallisticCoefficient,
        time_step: Numeric,
        muzzle_velocity: Numeric,
        scope_height: Numeric,
    ) -> Self {
        Self {
            weight: WeightMass::Grains(weight),
            caliber: Length::Inches(caliber),
            bc,
            drag_table: bc.table(),
            time_step: Time::Seconds(time_step),
            muzzle_velocity: Velocity::Fps(muzzle_velocity),
            scope_height: Length::Inches(scope_height),
        }
    }
    // Radius of projectile cross section in meters
    pub(crate) fn radius(&self) -> Numeric {
        Numeric::from(self.caliber.to_meters()) / 2.0
    }
    // Area of projectile in meters, used during drag force calculation
    pub(crate) fn area(&self) -> Numeric {
        PI * self.radius().powf(2.0)
    }
    // Mass of projectile in kgs, used during acceleration calculation in simulation iteration
    pub(crate) fn mass(&self) -> Numeric {
        self.weight.to_kgs().into()
    }
    // Sectional density of projectile, defined terms of lbs and inches, yet dimensionless
    pub(crate) fn sd(&self) -> Numeric {
        Numeric::from(self.weight.to_lbs()) / Numeric::from(self.caliber.to_inches()).powf(2.0)
    }
    // Form factor of projectile, calculated fro Ballistic Coefficient and Sectional Density (sd)
    pub(crate) fn i(&self) -> Numeric {
        self.sd() / Numeric::from(self.bc)
    }
    pub(crate) fn scope_height(&self) -> Vector3<Numeric> {
        Numeric::from(self.scope_height.to_meters()) * Vector3::y()
    }
}

// Environmental Conditions and other varialbe for simulation
pub struct Conditional {
    pub temperature: Temperature,  // Temperature (F)
    pub pressure: Pressure,        // Pressure (InHg)
    pub humidity: Numeric,         // Humidity (0-1)
    pub gravity: Vector3<Numeric>, // Gravity (m/s^2)
    pub wind_velocity: Velocity,   // Wind Velocity (miles/hour)
    pub wind_yaw: Numeric,         // Wind Angle (degrees)
    pub shooter_pitch: Numeric,    // Line of Sight angle (degrees)
    pub azimuth: Numeric,          // Bearing (0 North, 90 East) (degrees) (Coriolis/Eotvos Effect)
    pub lattitude: Numeric,        // Lattitude (Coriolis/Eotvos Effect)
}
impl Conditional {
    pub fn new(
        wind_velocity: Numeric,
        wind_yaw: Numeric,
        temperature: Numeric,
        pressure: Numeric,
        humidity: Numeric,
        shooter_pitch: Numeric,
        lattitude: Numeric,
        azimuth: Numeric,
    ) -> Self {
        Self {
            temperature: Temperature::F(temperature),
            pressure: Pressure::Inhg(pressure),
            humidity,
            gravity: GRAVITY * Vector3::y(),
            wind_velocity: Velocity::Mph(wind_velocity),
            wind_yaw: wind_yaw,
            shooter_pitch,
            lattitude,
            azimuth,
        }
    }
    pub(crate) fn lattitude(&self) -> Numeric {
        self.lattitude.to_radians()
    }
    pub(crate) fn shooter_pitch(&self) -> Numeric {
        self.shooter_pitch.to_radians()
    }
    // Negative indicates 90 degree wind is from east=>west
    // 0 degree wind is from north=>south (conventional)
    pub(crate) fn wind_yaw(&self) -> Numeric {
        -(self.wind_yaw.to_radians() - PI)
    }
    // Flip, since circle functions rotate counter-clockwise,
    // 90 degrees is east by compass bearing, but west(left) in trig
    pub(crate) fn azimuth(&self) -> Numeric {
        -self.azimuth.to_radians()
    }
    // Velocity vector of wind, right now calculated only for horizontal winds.  Can add another
    // factor, wind_pitch, to consider vertical wind components
    pub(crate) fn wind_velocity(&self) -> Vector3<Numeric> {
        Numeric::from(self.wind_velocity.to_mps())
            .mul(Vector3::x())
            .yaw(self.wind_yaw() + self.azimuth())
    }
    // Density of air, using pressure, humidity, and temperature
    pub fn rho(&self) -> Numeric {
        ((self.pd() * MOLAR_DRY) + (self.pv() * MOLAR_VAPOR)) / (UNIVERSAL_GAS * self.kelvin())
    }
    // Speed of sound at given air density and pressure
    pub(crate) fn c(&self) -> Numeric {
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
        Numeric::from(self.pressure.to_pascals())
    }
    // Temperature in celsius
    fn celsius(&self) -> Numeric {
        Numeric::from(self.temperature.to_celsius())
    }
    // Temperature in kelvin
    fn kelvin(&self) -> Numeric {
        Numeric::from(self.temperature.to_kelvin())
    }
}
