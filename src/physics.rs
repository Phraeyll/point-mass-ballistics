use nalgebra::Vector3;

use crate::{
    simulation::{Atmosphere, Flags, Projectile, Scope, Shooter, Wind},
    util::*,
    IterSimulation,
};

use std::ops::Mul;

const UNIVERSAL_GAS: Numeric = 8.314_459_8; // Universal gas constant (J/K*mol)
const MOLAR_DRY: Numeric = 0.028_964_4; // Molar mass of dry air (kg/mol)
const MOLAR_VAPOR: Numeric = 0.018_016; // Molar mass of water vapor (kg/mol)
const ADIABATIC_INDEX_AIR: Numeric = 1.4; // Adiabatic index of air, mostly diatomic gas
const ANGULAR_VELOCITY_EARTH: Numeric = 0.000_072_921_159; // Angular velocity of earth, (radians)

impl IterSimulation<'_> {
    fn projectile_mass(&self) -> Numeric {
        self.simulation.projectile.mass()
    }
    fn projectile_area(&self) -> Numeric {
        self.simulation.projectile.area()
    }
    fn i(&self) -> Numeric {
        self.simulation.projectile.i()
    }
    fn cd_table(&self) -> &FloatMap<Numeric> {
        &self.simulation.projectile.bc.table
    }
    fn wind_velocity(&self) -> Vector3<Numeric> {
        // Velocity vector of wind, only horizontal at the moment
        // Does not adjust according to line of sight, since most would measure wind
        // along relative bearing - I don't think many would factor in a 'downhill' wind for example
        // This would be interresting to think of, however.
        self.simulation
            .wind
            .velocity()
            .pivot_x(self.simulation.shooter.roll())
            .pivot_z(self.simulation.shooter.pitch())
            .pivot_y(self.simulation.shooter.yaw())
    }
    fn speed_of_sound(&self) -> Numeric {
        self.simulation.atmosphere.speed_of_sound()
    }
    fn rho(&self) -> Numeric {
        self.simulation.atmosphere.rho()
    }
    // Velocity vector, after impact from wind (actually from drag, not "being blown")
    // This is why the velocity from wind is subtracted, and vv is not used to find next velocity
    fn vv(&self) -> Vector3<Numeric> {
        self.velocity - self.wind_velocity()
    }
    // Velocity relative to speed of sound (c), with given atmospheric conditions
    fn mach(&self) -> Numeric {
        self.velocity.norm() / self.speed_of_sound()
    }
    // Coefficient of drag, as defined by a standard projectile depending on drag table used
    fn cd(&self) -> Numeric {
        self.i() * self.cd_table().lerp(self.mach()).expect("cd")
    }
    // Force of drag for given projectile, at given mach speed, with given conditions
    // Drag force is proportional to square of velocity and area of projectile, scaled
    // by a coefficient at mach speeds (approximately)
    fn drag_force(&self) -> Vector3<Numeric> {
        -0.5 * self.rho() * self.vv() * self.vv().norm() * self.cd() * self.projectile_area()
    }
    pub(crate) fn drag_acceleration(&self) -> Vector3<Numeric> {
        if self.simulation.flags.drag() {
            // Acceleration from drag force and gravity (F = ma)
            self.drag_force() / self.projectile_mass()
        } else {
            Vector3::zeros()
        }
    }
}

impl IterSimulation<'_> {
    fn omega(&self) -> Vector3<Numeric> {
        self.simulation.shooter.omega()
    }
    // Coriolis/Eotovos acceleration vector.  Accounts for Left/Right drift due to Earth's spin
    // This drift is always right (+z relative) in the northern hemisphere, regardless of initial bearing
    // This drive is always left (-z relative) in the southern hemisphere, regardless of initial bearing
    // Also accounts for elevation changes when launching projectils East/West, regardless of hemisphere
    // Bearing East results in higher elevation (+y absolute/relative)
    // Bearing West results in lower elevation (-y relative/absolute)
    pub(crate) fn coriolis_acceleration(&self) -> Vector3<Numeric> {
        if self.simulation.flags.coriolis() {
            -2.0 * self.omega().cross(&self.velocity)
        } else {
            Vector3::zeros()
        }
    }
}
impl IterSimulation<'_> {
    pub(crate) fn gravity_acceleration(&self) -> Vector3<Numeric> {
        if self.simulation.flags.gravity() {
            self.simulation.shooter.gravity()
        } else {
            Vector3::zeros()
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

impl Flags {
    pub(crate) fn coriolis(&self) -> bool {
        self.coriolis
    }
    pub(crate) fn drag(&self) -> bool {
        self.drag
    }
    pub(crate) fn gravity(&self) -> bool {
        self.gravity
    }
}
impl Projectile {
    // Radius of projectile cross section in meters
    fn radius(&self) -> Numeric {
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
    pub(crate) fn pitch(&self) -> Angle {
        self.pitch
    }
    pub(crate) fn yaw(&self) -> Angle {
        -self.yaw
    }
    pub(crate) fn roll(&self) -> Angle {
        -self.roll
    }
}
impl Shooter {
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
    pub(crate) fn omega(&self) -> Vector3<Numeric> {
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
    pub(crate) fn velocity(&self) -> Vector3<Numeric> {
        self.velocity
            .to_mps()
            .to_num()
            .mul(Vector3::x())
            .pivot_y(self.yaw())
            .pivot_z(self.pitch())
            .pivot_x(self.roll())
    }
}
