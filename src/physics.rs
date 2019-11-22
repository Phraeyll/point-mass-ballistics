use crate::{
    quantity,
    simulation::{Atmosphere, Flags, Projectile, Scope, Shooter, Simulation, Wind},
    util::{
        acceleration, angular_velocity, celsius, force, length, meter, meter_per_second,
        meter_per_second_squared, molar_mass, pascal, radian, radian_per_second, ratio,
        typenum::*, velocity, Acceleration, Angle, AngularVelocity, Area, Length, Mass,
        Ratio, MassDensity, MyQuantity, Numeric, Pressure, Velocity, ISQ, PI,
    },
    vectors::*,
};

// Universal gas constant (J/K*mol), aka,
const MOLAR_GAS_UNIVERSAL: MyQuantity<ISQ<P2, P1, N2, Z0, N1, N1, Z0>> = quantity!(8.314_462_618_153_24);

// Molar mass of dry air (kg/mol)
const MOLAR_MASS_DRY_AIR: MyQuantity<molar_mass::Dimension> = quantity!(0.028_964_4);

// Molar mass of water vapor (kg/mol)
const MOLAR_MASS_WATER_VAPOR: MyQuantity<molar_mass::Dimension> = quantity!(0.018_016);

// Angular velocity of earth, (radians)
const ANGULAR_VELOCITY_EARTH: MyQuantity<angular_velocity::Dimension> =
    quantity!(0.000_072_921_159);

// Adiabatic index of air, mostly diatomic gas
const ADIABATIC_INDEX_AIR: Numeric = 1.4;

// Drag
impl Simulation {
    // Velocity vector of wind, only horizontal at the moment
    // Does not adjust according to line of sight, since most would measure wind
    // along relative bearing - I don't think many would factor in a 'downhill' wind for example
    // This would be interresting to think of, however.
    fn wind_velocity(&self) -> MyVector3<velocity::Dimension> {
        self.wind
            .velocity()
            .pivot_x(self.shooter.roll())
            .pivot_z(self.shooter.pitch())
            .pivot_y(self.shooter.yaw())
    }
    // Velocity vector, after impact from wind (actually from drag, not "being blown")
    // This is why the velocity from wind is subtracted, and vv is not used to find next velocity
    fn vv(&self, velocity: &MyVector3<velocity::Dimension>) -> MyVector3<velocity::Dimension> {
        velocity - self.wind_velocity()
    }
    // Velocity relative to speed of sound (c), with given atmospheric conditions
    fn mach(&self, velocity: &MyVector3<velocity::Dimension>) -> Ratio {
        velocity.norm() / self.atmosphere.speed_of_sound()
    }
    // Coefficient of drag, as defined by a standard projectile depending on drag table used
    fn cd(&self, velocity: &MyVector3<velocity::Dimension>) -> Numeric {
        self.projectile.i().get::<ratio::ratio>()
            * self
                .projectile
                .bc
                .table()
                .lerp(self.mach(velocity).get::<ratio::ratio>())
                .expect("CD")
    }
    // Force of drag for given projectile, at given mach speed, with given conditions
    // Drag force is proportional to square of velocity and area of projectile, scaled
    // by a coefficient at mach speeds (approximately)
    fn drag_force(&self, velocity: &MyVector3<velocity::Dimension>) -> MyVector3<force::Dimension> {
        self.vv(velocity)
            * self.vv(velocity).norm()
            * self.atmosphere.rho()
            * self.projectile.area()
            * self.cd(velocity)
            * -0.5
    }
    pub(crate) fn drag_acceleration(
        &self,
        velocity: &MyVector3<velocity::Dimension>,
    ) -> MyVector3<acceleration::Dimension> {
        if self.flags.drag() {
            // Acceleration from drag force and gravity (F = ma)
            self.drag_force(velocity) / self.projectile.mass()
        } else {
            MyVector3::new(
                Acceleration::new::<meter_per_second_squared>(0.0),
                Acceleration::new::<meter_per_second_squared>(0.0),
                Acceleration::new::<meter_per_second_squared>(0.0),
            )
        }
    }
}

// Coriolis
impl Simulation {
    // Coriolis/Eotovos acceleration vector.  Accounts for Left/Right drift due to Earth's spin
    // This drift is always right (+z relative) in the northern hemisphere, regardless of initial bearing
    // This drive is always left (-z relative) in the southern hemisphere, regardless of initial bearing
    // Also accounts for elevation changes when launching projectils East/West, regardless of hemisphere
    // Bearing East results in higher elevation (+y absolute/relative)
    // Bearing West results in lower elevation (-y relative/absolute)
    pub(crate) fn coriolis_acceleration(
        &self,
        velocity: &MyVector3<velocity::Dimension>,
    ) -> MyVector3<acceleration::Dimension> {
        if self.flags.coriolis() {
            self.shooter.omega().cross(velocity) * -2.0
        } else {
            MyVector3::new(
                Acceleration::new::<meter_per_second_squared>(0.0),
                Acceleration::new::<meter_per_second_squared>(0.0),
                Acceleration::new::<meter_per_second_squared>(0.0),
            )
        }
    }
}

//Gravity
impl Simulation {
    pub(crate) fn gravity_acceleration(&self) -> MyVector3<acceleration::Dimension> {
        if self.flags.gravity() {
            self.shooter.gravity()
        } else {
            MyVector3::new(
                Acceleration::new::<meter_per_second_squared>(0.0),
                Acceleration::new::<meter_per_second_squared>(0.0),
                Acceleration::new::<meter_per_second_squared>(0.0),
            )
        }
    }
}

// Helpers - maybe some of these should be moved?
impl Atmosphere {
    // Density of air, using pressure, humidity, and temperature
    pub(crate) fn rho(&self) -> MassDensity {
        (((self.pd() * MOLAR_MASS_DRY_AIR) + (self.pv() * MOLAR_MASS_WATER_VAPOR))
            / (MOLAR_GAS_UNIVERSAL * self.temperature))
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
impl Projectile {
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
    fn sd(&self) -> MyQuantity<ISQ<N2, P1, Z0, Z0, Z0, Z0, Z0>> {
        self.weight / self.caliber.powi(P2::new())
    }
    // Form factor of projectile, calculated fro Ballistic Coefficient and Sectional Density (sd)
    fn i(&self) -> Ratio {
        self.sd() / self.bc.value
    }
    pub(crate) fn velocity(&self, scope: &Scope) -> MyVector3<velocity::Dimension> {
        MyVector3::new(
            self.velocity,
            Velocity::new::<meter_per_second>(0.0),
            Velocity::new::<meter_per_second>(0.0),
        )
        .pivot_y(scope.yaw())
        .pivot_z(scope.pitch())
    }
}
impl Scope {
    pub(crate) fn position(&self) -> MyVector3<length::Dimension> {
        MyVector3::new(Length::new::<meter>(0.0), self.height, self.offset)
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
    fn gravity(&self) -> MyVector3<acceleration::Dimension> {
        MyVector3::new(
            Acceleration::new::<meter_per_second_squared>(0.0),
            self.gravity,
            Acceleration::new::<meter_per_second_squared>(0.0),
        )
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
    fn omega(&self) -> MyVector3<angular_velocity::Dimension> {
        MyVector3::new(
            ANGULAR_VELOCITY_EARTH,
            AngularVelocity::new::<radian_per_second>(0.0),
            AngularVelocity::new::<radian_per_second>(0.0),
        )
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
    fn velocity(&self) -> MyVector3<velocity::Dimension> {
        MyVector3::new(
            self.velocity,
            Velocity::new::<meter_per_second>(0.0),
            Velocity::new::<meter_per_second>(0.0),
        )
        .pivot_y(self.yaw())
        .pivot_z(self.pitch())
        .pivot_x(self.roll())
    }
}
