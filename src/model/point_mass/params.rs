pub use super::dragtables::*;
use nalgebra::Vector3;

use crate::util::*;

use std::ops::Mul;

const GRAVITY: Numeric = -9.806_65; // Local gravity in m/s
const UNIVERSAL_GAS: Numeric = 8.314_459_8; // Universal gas constant (J/K*mol)
const MOLAR_DRY: Numeric = 0.028_964_4; // Molar mass of dry air (kg/mol)
const MOLAR_VAPOR: Numeric = 0.018_016; // Molar mass of water vapor (kg/mol)
const ADIABATIC_INDEX_AIR: Numeric = 1.4; // Adiabatic index of air, mostly diatomic gas
const ANGULAR_VELOCITY_EARTH: Numeric = 0.000_072_921_159; // Angular velocity of earth, (radians)

pub struct Projectile {
    weight: WeightMass,                       // Weight (grains)
    caliber: Length,                          // Caliber (inches)
    bc: BallisticCoefficient,                 // Ballistic Coefficient
    pub(crate) drag_table: FloatMap<Numeric>, // Drag Function DragTable
    pub(crate) velocity: Velocity,            // Initial velocity (ft/s)
}
impl Projectile {
    pub fn new(
        weight: Numeric,
        caliber: Numeric,
        bc: BallisticCoefficient,
        velocity: Numeric,
    ) -> Self {
        Self {
            weight: WeightMass::Grains(weight),
            caliber: Length::Inches(caliber),
            bc,
            drag_table: bc.table(),
            velocity: Velocity::Fps(velocity),
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
    fn sd(&self) -> Numeric {
        Numeric::from(self.weight.to_lbs()) / Numeric::from(self.caliber.to_inches()).powf(2.0)
    }
    // Form factor of projectile, calculated fro Ballistic Coefficient and Sectional Density (sd)
    pub(crate) fn i(&self) -> Numeric {
        self.sd() / Numeric::from(self.bc)
    }
    pub(crate) fn velocity(&self) -> Vector3<Numeric> {
        Numeric::from(self.velocity.to_mps()) * Vector3::x()
    }
}

pub struct Scope {
    height: Length, // Scope Height (inches)
}
impl Scope {
    pub fn new(height: Numeric) -> Self {
        Self {
            height: Length::Inches(height),
        }
    }
    pub(crate) fn height(&self) -> Vector3<Numeric> {
        Numeric::from(self.height.to_meters()) * Vector3::y()
    }
}

pub struct Wind {
    velocity: Velocity, // Wind Velocity (miles/hour)
    yaw: Numeric,       // Wind Angle (degrees)
}
impl Wind {
    pub fn new(velocity: Numeric, yaw: Numeric) -> Self {
        Self {
            velocity: Velocity::Mph(velocity),
            yaw: yaw,
        }
    }
    // Negative indicates 90 degree wind is from east=>west
    // 0 degree wind is from north=>south (conventional)
    pub(crate) fn yaw(&self) -> Numeric {
        -(self.yaw.to_radians() - PI)
    }
    pub(crate) fn velocity(&self) -> Vector3<Numeric> {
        Numeric::from(self.velocity.to_mps()) * Vector3::x()
    }
}

// Environmental Conditions and other varialbe for simulation
pub struct Atmosphere {
    temperature: Temperature, // Temperature (F)
    pressure: Pressure,       // Pressure (InHg)
    humidity: Numeric,        // Humidity (0-1)
}
impl Atmosphere {
    pub fn new(temperature: Numeric, pressure: Numeric, humidity: Numeric) -> Self {
        Self {
            temperature: Temperature::F(temperature),
            pressure: Pressure::Inhg(pressure),
            humidity,
        }
    }
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

pub struct Conditions {
    line_of_sight: Numeric, // Line of Sight angle (degrees)
    azimuth: Numeric,       // Bearing (0 North, 90 East) (degrees) (Coriolis/Eotvos Effect)
    lattitude: Numeric,     // Lattitude (Coriolis/Eotvos Effect)
    gravity: Acceleration,  // Gravity (m/s^2)
}
impl Conditions {
    pub fn new(
        line_of_sight: Numeric,
        lattitude: Numeric,
        azimuth: Numeric,
        gravity: Option<Numeric>,
    ) -> Self {
        Self {
            gravity: match gravity {
                Some(gravity) => Acceleration::Fps2(gravity),
                None => Acceleration::Mps2(GRAVITY),
            },
            line_of_sight,
            lattitude,
            azimuth,
        }
    }
    pub(crate) fn gravity(&self) -> Vector3<Numeric> {
        Numeric::from(self.gravity.to_mps2()) * Vector3::y()
    }
    fn lattitude(&self) -> Numeric {
        self.lattitude.to_radians()
    }
    pub(crate) fn line_of_sight(&self) -> Numeric {
        self.line_of_sight.to_radians()
    }
    // Flip, since circle functions rotate counter-clockwise,
    // 90 degrees is east by compass bearing, but west(left) in trig
    pub(crate) fn azimuth(&self) -> Numeric {
        -self.azimuth.to_radians()
    }
    // Angular velocity vector of earth, at current lattitude
    // Can be thought of as vector pointing along y axis from center of earth, rolled along
    // lines of lattitude, as represented here now
    pub(crate) fn omega(&self) -> Vector3<Numeric> {
        ANGULAR_VELOCITY_EARTH
            .mul(Vector3::x())
            .pitch(self.lattitude())
    }
}
