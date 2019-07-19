use crate::{
    simulation::{Atmosphere, Flags, Projectile, Scope, Shooter, Simulation, Wind},
    util::*,
};

use std::ops::Mul;

use nalgebra::Vector3;

const UNIVERSAL_GAS: Numeric = 8.314_459_8; // Universal gas constant (J/K*mol)
const MOLAR_DRY: Numeric = 0.028_964_4; // Molar mass of dry air (kg/mol)
const MOLAR_VAPOR: Numeric = 0.018_016; // Molar mass of water vapor (kg/mol)
const ADIABATIC_INDEX_AIR: Numeric = 1.4; // Adiabatic index of air, mostly diatomic gas
const ANGULAR_VELOCITY_EARTH: Numeric = 0.000_072_921_159; // Angular velocity of earth, (radians)

// Drag
impl Simulation<'_> {
    // Velocity vector of wind, only horizontal at the moment
    // Does not adjust according to line of sight, since most would measure wind
    // along relative bearing - I don't think many would factor in a 'downhill' wind for example
    // This would be interresting to think of, however.
    fn wind_velocity(&self) -> Vector3<Numeric> {
        self.wind
            .velocity()
            .pivot_x(self.shooter.roll())
            .pivot_z(self.shooter.pitch())
            .pivot_y(self.shooter.yaw())
    }
    // Velocity vector, after impact from wind (actually from drag, not "being blown")
    // This is why the velocity from wind is subtracted, and vv is not used to find next velocity
    fn vv(&self, velocity: &Vector3<Numeric>) -> Vector3<Numeric> {
        velocity - self.wind_velocity()
    }
    // Velocity relative to speed of sound (c), with given atmospheric conditions
    fn mach(&self, velocity: &Vector3<Numeric>) -> Numeric {
        velocity.norm() / self.atmosphere.speed_of_sound()
    }
    // Coefficient of drag, as defined by a standard projectile depending on drag table used
    fn cd(&self, velocity: &Vector3<Numeric>) -> Numeric {
        self.projectile.i() * self.projectile.bc.table.lerp(self.mach(velocity)).expect("cd")
    }
    // Force of drag for given projectile, at given mach speed, with given conditions
    // Drag force is proportional to square of velocity and area of projectile, scaled
    // by a coefficient at mach speeds (approximately)
    fn drag_force(&self, velocity: &Vector3<Numeric>) -> Vector3<Numeric> {
        -0.5 * self.atmosphere.rho()
            * self.vv(velocity)
            * self.vv(velocity).norm()
            * self.cd(velocity)
            * self.projectile.area()
    }
    pub(crate) fn drag_acceleration(&self, velocity: &Vector3<Numeric>) -> Vector3<Numeric> {
        if self.flags.drag() {
            // Acceleration from drag force and gravity (F = ma)
            self.drag_force(velocity) / self.projectile.mass()
        } else {
            Vector3::zeros()
        }
    }
}

// Coriolis
impl Simulation<'_> {
    // Coriolis/Eotovos acceleration vector.  Accounts for Left/Right drift due to Earth's spin
    // This drift is always right (+z relative) in the northern hemisphere, regardless of initial bearing
    // This drive is always left (-z relative) in the southern hemisphere, regardless of initial bearing
    // Also accounts for elevation changes when launching projectils East/West, regardless of hemisphere
    // Bearing East results in higher elevation (+y absolute/relative)
    // Bearing West results in lower elevation (-y relative/absolute)
    pub(crate) fn coriolis_acceleration(&self, velocity: &Vector3<Numeric>) -> Vector3<Numeric> {
        if self.flags.coriolis() {
            -2.0 * self.shooter.omega().cross(velocity)
        } else {
            Vector3::zeros()
        }
    }
}

//Gravity
impl Simulation<'_> {
    pub(crate) fn gravity_acceleration(&self) -> Vector3<Numeric> {
        if self.flags.gravity() {
            self.shooter.gravity()
        } else {
            Vector3::zeros()
        }
    }
}

// Helpers - maybe some of these should be moved?
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

impl Flags {
    fn coriolis(&self) -> bool {
        self.coriolis
    }
    fn drag(&self) -> bool {
        self.drag
    }
    fn gravity(&self) -> bool {
        self.gravity
    }
}
impl Projectile<'_> {
    // Radius of projectile cross section in meters
    fn radius(&self) -> Numeric {
        self.caliber.to_meters().to_num() / 2.0
    }
    // Area of projectile in meters, used during drag force calculation
    fn area(&self) -> Numeric {
        PI * self.radius().powf(2.0)
    }
    // Mass of projectile in kgs, used during acceleration calculation in get_simulation().iteration
    pub(crate) fn mass(&self) -> Numeric {
        self.weight.to_kgs().into()
    }
    // Sectional density of projectile, defined terms of lbs and inches, yet dimensionless
    fn sd(&self) -> Numeric {
        self.weight.to_lbs().to_num() / self.caliber.to_inches().to_num().powf(2.0)
    }
    // Form factor of projectile, calculated fro Ballistic Coefficient and Sectional Density (sd)
    fn i(&self) -> Numeric {
        self.sd() / self.bc.value
    }
    pub(crate) fn velocity(&self, scope: &Scope) -> Vector3<Numeric> {
        self.velocity
            .to_mps()
            .to_num()
            .mul(Vector3::x())
            .pivot_y(scope.yaw())
            .pivot_z(scope.pitch())
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
    fn pitch(&self) -> Angle {
        self.pitch
    }
    fn yaw(&self) -> Angle {
        -self.yaw
    }
    pub(crate) fn roll(&self) -> Angle {
        -self.roll
    }
}
impl Shooter {
    fn gravity(&self) -> Vector3<Numeric> {
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
    pub(crate) fn yaw(&self) -> Angle {
        -self.yaw
    }
    pub(crate) fn pitch(&self) -> Angle {
        self.pitch
    }
    pub(crate) fn roll(&self) -> Angle {
        -self.roll
    }
    // Angular velocity vector of earth, at current lattitude
    // Can be thought of as vector from center of earth, pointing
    // to lines of lattitude.  Maximum effect at +/-90 degrees (poles)
    fn omega(&self) -> Vector3<Numeric> {
        ANGULAR_VELOCITY_EARTH
            .mul(Vector3::x())
            .pivot_z(self.lattitude)
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
    fn yaw(&self) -> Angle {
        -self.yaw + Angle::Radians(PI)
    }
    fn pitch(&self) -> Angle {
        self.pitch
    }
    fn roll(&self) -> Angle {
        self.roll
    }
    fn velocity(&self) -> Vector3<Numeric> {
        self.velocity
            .to_mps()
            .to_num()
            .mul(Vector3::x())
            .pivot_y(self.yaw())
            .pivot_z(self.pitch())
            .pivot_x(self.roll())
    }
}
