use crate::{
    simulation::{Atmosphere, Flags, Projectile, Scope, Shooter, Simulation, Wind},
    util::{
        celsius, inch, kilogram, kilogram_per_cubic_meter, meter, meter_per_second,
        meter_per_second_squared, molar_mass, nalgebra_helpers::*, pascal, pound, radian,
        square_meter, typenum::*, Angle, Area, Length, Mass, MassDensity, Numeric, Pressure,
        Quantity, Velocity, ISQ, PI, SI,
    },
};

use std::{marker::PhantomData, ops::Mul};

use nalgebra::Vector3;

// Universal gas constant (J/K*mol)
type GasDimension = ISQ<P2, P1, N2, Z0, N1, N1, Z0>;
const UNIVERSAL_GAS: Quantity<GasDimension, SI<Numeric>, Numeric> = Quantity {
    dimension: PhantomData,
    units: PhantomData,
    value: 8.314_462_618_153_24,
};
// Molar mass of dry air (kg/mol)
const MOLAR_DRY: Quantity<molar_mass::Dimension, SI<Numeric>, Numeric> = Quantity {
    dimension: PhantomData,
    units: PhantomData,
    value: 0.028_964_4,
};
// Molar mass of water vapor (kg/mol)
const MOLAR_VAPOR: Quantity<molar_mass::Dimension, SI<Numeric>, Numeric> = Quantity {
    dimension: PhantomData,
    units: PhantomData,
    value: 0.018_016,
};
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
        velocity.norm() / Velocity::get::<meter_per_second>(&self.atmosphere.speed_of_sound())
    }
    // Coefficient of drag, as defined by a standard projectile depending on drag table used
    fn cd(&self, velocity: &Vector3<Numeric>) -> Numeric {
        self.projectile.i()
            * self
                .projectile
                .bc
                .table
                .expect("BC")
                .lerp(self.mach(velocity))
                .expect("CD")
    }
    // Force of drag for given projectile, at given mach speed, with given conditions
    // Drag force is proportional to square of velocity and area of projectile, scaled
    // by a coefficient at mach speeds (approximately)
    fn drag_force(&self, velocity: &Vector3<Numeric>) -> Vector3<Numeric> {
        -0.5 * self.atmosphere.rho().get::<kilogram_per_cubic_meter>()
            * self.vv(velocity)
            * self.vv(velocity).norm()
            * self.cd(velocity)
            * self.projectile.area().get::<square_meter>()
    }
    pub(crate) fn drag_acceleration(&self, velocity: &Vector3<Numeric>) -> Vector3<Numeric> {
        if self.flags.drag() {
            // Acceleration from drag force and gravity (F = ma)
            self.drag_force(velocity) / self.projectile.mass().get::<kilogram>()
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
    pub(crate) fn rho(&self) -> MassDensity {
        (((self.pd() * MOLAR_DRY) + (self.pv() * MOLAR_VAPOR)) / (UNIVERSAL_GAS * self.temperature))
    }
    // Speed of sound at given air density and pressure
    pub(crate) fn speed_of_sound(&self) -> Velocity {
        (ADIABATIC_INDEX_AIR * (self.pressure / self.rho())).sqrt()
    }
    // Pressure of water vapor, Arden Buck equation
    fn pv(&self) -> Pressure {
        Pressure::new::<pascal>(
            self.humidity
                * 611.21
                * ((18.678 - (self.celsius() / 234.5))
                    * (self.celsius() / (257.14 + self.celsius())))
                .exp(),
        )
    }
    // Pressure of dry air
    fn pd(&self) -> Pressure {
        self.pressure - self.pv()
    }
    // Temperature in celsius
    fn celsius(&self) -> Numeric {
        self.temperature.get::<celsius>()
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
    fn radius(&self) -> Length {
        self.caliber / 2.0
    }
    // Area of projectile in meters, used during drag force calculation
    fn area(&self) -> Area {
        PI * self.radius().powi(P2::new())
    }
    // Mass of projectile in kgs, used during acceleration calculation in get_simulation().iteration
    pub(crate) fn mass(&self) -> Mass {
        self.weight
    }
    // Sectional density of projectile, defined terms of lbs and inches, yet dimensionless
    fn sd(&self) -> Numeric {
        self.weight.get::<pound>() / self.caliber.get::<inch>().powf(2.0)
    }
    // Form factor of projectile, calculated fro Ballistic Coefficient and Sectional Density (sd)
    fn i(&self) -> Numeric {
        self.sd() / self.bc.value
    }
    pub(crate) fn velocity(&self, scope: &Scope) -> Vector3<Numeric> {
        self.velocity
            .get::<meter_per_second>()
            .mul(Vector3::x())
            .pivot_y(scope.yaw())
            .pivot_z(scope.pitch())
    }
}
impl Scope {
    pub(crate) fn position(&self) -> Vector3<Numeric> {
        Vector3::new(0.0, self.height.get::<meter>(), self.offset.get::<meter>())
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
        self.gravity
            .get::<meter_per_second_squared>()
            .mul(Vector3::y())
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
        -self.yaw + Angle::new::<radian>(PI)
    }
    fn pitch(&self) -> Angle {
        self.pitch
    }
    fn roll(&self) -> Angle {
        self.roll
    }
    fn velocity(&self) -> Vector3<Numeric> {
        self.velocity
            .get::<meter_per_second>()
            .mul(Vector3::x())
            .pivot_y(self.yaw())
            .pivot_z(self.pitch())
            .pivot_x(self.roll())
    }
}
