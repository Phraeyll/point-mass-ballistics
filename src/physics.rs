use crate::{
    projectiles::Projectile,
    simulation::{Atmosphere, Flags, Scope, Shooter, Simulation, Wind},
    units::{
        acceleration, angular_velocity, celsius, force, meter_per_second, meter_per_second_squared,
        my_quantity, pascal, radian_per_second, ratio, velocity, Acceleration, Angle,
        AngularVelocity, MassDensity, MolarHeatCapacity, MolarMass, MyQuantity, Pressure, Ratio,
        Velocity,
    },
    vectors::{Cross, MyVector3, Norm, Vectors},
    Numeric,
};

// Drag
impl<T> Simulation<T>
where
    T: Projectile,
{
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
    fn vv(&self, velocity: MyVector3<velocity::Dimension>) -> MyVector3<velocity::Dimension> {
        velocity - self.wind_velocity()
    }
    // Velocity relative to speed of sound (c), with given atmospheric conditions
    fn mach(&self, velocity: MyVector3<velocity::Dimension>) -> Ratio {
        velocity.norm() / self.atmosphere.speed_of_sound()
    }
    // Coefficient of drag, as defined by a standard projectile depending on drag table used
    fn cd(&self, velocity: MyVector3<velocity::Dimension>) -> Ratio {
        self.projectile.i()
            * self
                .projectile
                .cd(self.mach(velocity).get::<ratio::ratio>())
                .expect("CD")
    }
    // Force of drag for given projectile, at given mach speed, with given conditions
    // Drag force is proportional to square of velocity and area of projectile, scaled
    // by a coefficient at mach speeds (approximately)
    fn drag_force(&self, velocity: MyVector3<velocity::Dimension>) -> MyVector3<force::Dimension> {
        self.vv(velocity)
            * self.vv(velocity).norm()
            * self.atmosphere.rho()
            * self.projectile.area()
            * self.cd(velocity)
            * -0.5
    }
    pub(crate) fn drag_acceleration(
        &self,
        velocity: MyVector3<velocity::Dimension>,
    ) -> MyVector3<acceleration::Dimension> {
        if self.flags.drag() {
            // Acceleration from drag force and gravity (F = ma)
            self.drag_force(velocity) * (1.0 / self.projectile.mass())
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
impl<T> Simulation<T> {
    // Coriolis/Eotovos acceleration vector.  Accounts for Left/Right drift due to Earth's spin
    // This drift is always right (+z relative) in the northern hemisphere, regardless of initial bearing
    // This drive is always left (-z relative) in the southern hemisphere, regardless of initial bearing
    // Also accounts for elevation changes when launching projectils East/West, regardless of hemisphere
    // Bearing East results in higher elevation (+y absolute/relative)
    // Bearing West results in lower elevation (-y relative/absolute)
    pub(crate) fn coriolis_acceleration(
        &self,
        velocity: MyVector3<velocity::Dimension>,
    ) -> MyVector3<acceleration::Dimension> {
        if self.flags.coriolis() {
            self.shooter.omega().cross(&velocity) * -2.0
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
impl<T> Simulation<T> {
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
    // Universal gas constant (J/K*mol)
    const MOLAR_GAS_UNIVERSAL: MolarHeatCapacity = my_quantity!(8.314_462_618_153_24);

    // Molar mass of dry air (kg/mol)
    const MOLAR_MASS_DRY_AIR: MolarMass = my_quantity!(0.028_964_4);

    // Molar mass of water vapor (kg/mol)
    const MOLAR_MASS_WATER_VAPOR: MolarMass = my_quantity!(0.018_016);

    // Adiabatic index of air, mostly diatomic gas
    const ADIABATIC_INDEX_AIR: Numeric = 1.4;

    // Density of air, using pressure, humidity, and temperature
    pub(crate) fn rho(&self) -> MassDensity {
        ((self.pd() * Self::MOLAR_MASS_DRY_AIR) + (self.pv() * Self::MOLAR_MASS_WATER_VAPOR))
            / (Self::MOLAR_GAS_UNIVERSAL * self.temperature)
    }
    // Speed of sound at given air density and pressure
    pub(crate) fn speed_of_sound(&self) -> Velocity {
        (Self::ADIABATIC_INDEX_AIR * (self.pressure / self.rho())).sqrt()
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
impl Scope {
    pub(crate) fn pitch(&self) -> Angle {
        self.pitch
    }
    pub(crate) fn yaw(&self) -> Angle {
        -self.yaw
    }
    pub(crate) fn roll(&self) -> Angle {
        self.roll
    }
}
impl Shooter {
    // Angular velocity of earth, (radians)
    const ANGULAR_VELOCITY_EARTH: AngularVelocity = my_quantity!(0.000_072_921_159);

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
            Self::ANGULAR_VELOCITY_EARTH,
            AngularVelocity::new::<radian_per_second>(0.0),
            AngularVelocity::new::<radian_per_second>(0.0),
        )
        .pivot_z(self.lattitude)
    }
}
impl Wind {
    // This vector indicates direction of wind flow, not source of wind
    fn yaw(&self) -> Angle {
        self.yaw
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
