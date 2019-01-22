use nalgebra::Vector3;

use crate::util::*;
use crate::model::builder::*;

use std::ops::Mul;

pub(crate) const GRAVITY: Numeric = -9.806_65; // Local gravity in m/s
const UNIVERSAL_GAS: Numeric = 8.314_459_8; // Universal gas constant (J/K*mol)
const MOLAR_DRY: Numeric = 0.028_964_4; // Molar mass of dry air (kg/mol)
const MOLAR_VAPOR: Numeric = 0.018_016; // Molar mass of water vapor (kg/mol)
const ADIABATIC_INDEX_AIR: Numeric = 1.4; // Adiabatic index of air, mostly diatomic gas
const ANGULAR_VELOCITY_EARTH: Numeric = 0.000_072_921_159; // Angular velocity of earth, (radians)

#[derive(Debug)]
pub struct Simulation<'p> {
    pub(crate) flags: &'p Flags,
    pub(crate) projectile: &'p Projectile,
    pub(crate) scope: &'p Scope,
    pub(crate) conditions: &'p Conditions,
    pub(crate) angles: Angles,
    pub(crate) time_step: Time,
}
impl<'p> Simulation<'p> {
    pub(crate) fn new(
        flags: &'p Flags,
        projectile: &'p Projectile,
        scope: &'p Scope,
        conditions: &'p Conditions,
        angles: Angles,
        time_step: Time,
    ) -> Self {
        Self {
            projectile,
            scope,
            conditions,
            flags,
            angles,
            time_step,
        }
    }
    // Rotated velocity vector, accounts for muzzle/shooter pitch, and yaw (bearing)
    // Start with velocity value along X unit vector
    pub(crate) fn absolute_projectile_velocity(&self) -> Vector3<Numeric> {
        self.projectile
            .velocity(self.angles.pitch, self.angles.yaw)
            .pivot_z(self.conditions.other.line_of_sight)
            .pivot_y(self.conditions.other.corrected_azimuth())
    }
    // Velocity vector of wind, only horizontal at the moment
    // Does not adjust according to line of sight, since most would measure wind
    // along relative bearing - I don't think many would factor in a 'downhill' wind for example
    // This would be interresting to think of, however.
    pub(crate) fn absolute_wind_velocity(&self) -> Vector3<Numeric> {
        self.conditions
            .wind
            .velocity()
            .pivot_y(self.conditions.other.corrected_azimuth())
    }
}
impl BallisticCoefficient {
    pub fn value(&self) -> Numeric {
        self.value
    }
    pub fn table(&self) -> &FloatMap<Numeric> {
        &self.table
    }
    pub fn kind(&self) -> BallisticCoefficientKind {
        self.kind
    }
}
impl Projectile {
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
    pub(crate) fn sd(&self) -> Numeric {
        self.weight.to_lbs().to_num() / self.caliber.to_inches().to_num().powf(2.0)
    }
    // Form factor of projectile, calculated fro Ballistic Coefficient and Sectional Density (sd)
    pub(crate) fn i(&self) -> Numeric {
        self.sd() / self.bc.value()
    }
    pub(crate) fn velocity(&self, muzzle_pitch: Angle, muzzle_yaw: Angle) -> Vector3<Numeric> {
        self.velocity
            .to_mps()
            .to_num()
            .mul(Vector3::x())
            .pivot_z(muzzle_pitch)
            .pivot_y(muzzle_yaw)
    }
}
impl Scope {
    pub(crate) fn position(&self) -> Vector3<Numeric> {
        Vector3::new(
            0.0,
            self.height.to_meters().to_num(),
            self.offset.to_meters().to_num(),
        )
    }
}
impl Wind {
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
    pub(crate) fn corrected_yaw(&self) -> Angle {
        Angle::Radians(-self.yaw.to_radians().to_num() + PI)
    }
    pub(crate) fn velocity(&self) -> Vector3<Numeric> {
        self.velocity
            .to_mps()
            .to_num()
            .mul(Vector3::x())
            .pivot_y(self.corrected_yaw())
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
    pub(crate) fn pv(&self) -> Numeric {
        self.humidity
            * 611.21
            * ((18.678 - (self.celsius() / 234.5)) * (self.celsius() / (257.14 + self.celsius())))
                .exp()
    }
    // Pressure of dry air
    pub(crate) fn pd(&self) -> Numeric {
        self.pa() - self.pv()
    }
    // Total air pressure in pascals
    pub(crate) fn pa(&self) -> Numeric {
        self.pressure.to_pascals().to_num()
    }
    // Temperature in celsius
    pub(crate) fn celsius(&self) -> Numeric {
        self.temperature.to_celsius().to_num()
    }
    // Temperature in kelvin
    pub(crate) fn kelvin(&self) -> Numeric {
        self.temperature.to_kelvin().to_num()
    }
}

impl Other {
    pub(crate) fn gravity(&self) -> Vector3<Numeric> {
        self.gravity.to_mps2().to_num().mul(Vector3::y())
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
    pub(crate) fn corrected_azimuth(&self) -> Angle {
        Angle::Radians(-self.azimuth.to_radians().to_num())
    }
    // Angular velocity vector of earth, at current lattitude
    // Can be thought of as vector from center of earth, pointing
    // to lines of lattitude.  Maximum effect at +/-90 degrees (poles)
    pub(crate) fn omega(&self) -> Vector3<Numeric> {
        ANGULAR_VELOCITY_EARTH
            .mul(Vector3::x())
            .pivot_z(self.lattitude)
    }
}
