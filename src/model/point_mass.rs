pub use self::dragtables::*;

use nalgebra::Vector3;

use crate::util::*;

use std::ops::Mul;

pub mod builder;
#[allow(clippy::approx_constant)]
mod dragtables;
#[allow(clippy::float_cmp)]
pub mod iter;
pub mod table;
#[allow(clippy::float_cmp)]
pub(crate) mod zero;

const GRAVITY: Numeric = -9.806_65; // Local gravity in m/s
const UNIVERSAL_GAS: Numeric = 8.314_459_8; // Universal gas constant (J/K*mol)
const MOLAR_DRY: Numeric = 0.028_964_4; // Molar mass of dry air (kg/mol)
const MOLAR_VAPOR: Numeric = 0.018_016; // Molar mass of water vapor (kg/mol)
const ADIABATIC_INDEX_AIR: Numeric = 1.4; // Adiabatic index of air, mostly diatomic gas
const ANGULAR_VELOCITY_EARTH: Numeric = 0.000_072_921_159; // Angular velocity of earth, (radians)

pub struct Simulation<'p> {
    projectile: &'p Projectile,
    scope: &'p Scope,
    conditions: &'p Conditions<'p>,
    muzzle_pitch: Numeric,
    zero_distance: Length,
    time_step: Time,
    zero: Numeric,
}
impl<'p> Simulation<'p> {
    pub(crate) fn new(
        projectile: &'p Projectile,
        scope: &'p Scope,
        conditions: &'p Conditions<'p>,
        muzzle_pitch: Numeric,
        zero_distance: Numeric,
        time_step: Numeric,
        zero: Numeric,
    ) -> Self {
        Self {
            projectile,
            scope,
            conditions,
            muzzle_pitch,
            zero_distance: Length::Yards(zero_distance),
            time_step: Time::Seconds(time_step),
            zero,
        }
    }
    // Rotated velocity vector, accounts for muzzle/shooter pitch, and yaw (bearing)
    // Start with velocity value along X unit vector
    fn muzzle_velocity_vector(&self) -> Vector3<Numeric> {
        self.projectile
            .velocity()
            .pitch(self.conditions.other.line_of_sight() + self.muzzle_pitch)
            .yaw(self.conditions.other.azimuth())
    }
    // Velocity vector of wind, right now calculated only for horizontal winds.  Can add another
    // factor, wind_pitch, to consider vertical wind components
    fn wind_velocity_vector(&self) -> Vector3<Numeric> {
        self.conditions
            .wind
            .velocity()
            .yaw(self.conditions.wind.yaw() + self.conditions.other.azimuth())
    }
}

pub struct Conditions<'c> {
    pub(crate) wind: &'c Wind,
    pub(crate) atmosphere: &'c Atmosphere,
    pub(crate) other: &'c Other,
}
impl<'c> Conditions<'c> {
    pub fn new(wind: &'c Wind, atmosphere: &'c Atmosphere, other: &'c Other) -> Self {
        Self {
            wind,
            atmosphere,
            other,
        }
    }
}

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
        self.caliber.to_meters().to_num() / 2.0
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
        self.weight.to_lbs().to_num() / self.caliber.to_inches().to_num().powf(2.0)
    }
    // Form factor of projectile, calculated fro Ballistic Coefficient and Sectional Density (sd)
    pub(crate) fn i(&self) -> Numeric {
        self.sd() / self.bc.to_num()
    }
    pub(crate) fn velocity(&self) -> Vector3<Numeric> {
        self.velocity.to_mps().to_num() * Vector3::x()
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
        self.height.to_meters().to_num() * Vector3::y()
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
    // This vector indicates direction of wind flow, not source of wind
    // so rotate by PI (adding or subtraction should have the same affect)
    // Negative indicates 90 degree wind is from east=>west
    // 0 degree wind is from north=>south (conventional)
    //        (0)
    //         ^
    //         |
    // (+90) <---> (-90)
    //         |
    //         v
    //       (180)
    //
    //  {after rotation(+ PI)}
    //
    //       (180)
    //         ^
    //         |
    // (-90) <---> (+90)
    //         |
    //         v
    //        (0)
    //
    //  {after negation(-)}
    //
    //       (180)
    //         ^
    //         |
    // (+90) <---> (-90)
    //         |
    //         v
    //        (0)
    pub(crate) fn yaw(&self) -> Numeric {
        -(self.yaw.to_radians() + PI)
    }
    pub(crate) fn velocity(&self) -> Vector3<Numeric> {
        self.velocity.to_mps().to_num() * Vector3::x()
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

pub struct Other {
    line_of_sight: Numeric, // Line of Sight angle (degrees)
    azimuth: Numeric,       // Bearing (0 North, 90 East) (degrees) (Coriolis/Eotvos Effect)
    lattitude: Numeric,     // Lattitude (Coriolis/Eotvos Effect)
    gravity: Acceleration,  // Gravity (m/s^2)
}
impl Other {
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
        self.gravity.to_mps2().to_num() * Vector3::y()
    }
    fn lattitude(&self) -> Numeric {
        self.lattitude.to_radians()
    }
    pub(crate) fn line_of_sight(&self) -> Numeric {
        self.line_of_sight.to_radians()
    }
    // Flip, since circle functions rotate counter-clockwise,
    // 90 degrees is east by compass bearing, but west(left) in trig
    //        (0)
    //         ^
    //         |
    // (+90) <---> (-90)
    //         |
    //         v
    //       (180)
    //
    //  {after negation(-)}
    //
    //        (0)
    //         ^
    //         |
    // (-90) <---> (+90)
    //         |
    //         v
    //       (180)
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
