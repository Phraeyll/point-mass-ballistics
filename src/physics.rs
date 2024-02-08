use crate::{
    consts::PI,
    simulation::{Atmosphere, Projectile, Scope, Shooter, Simulation, Wind},
    units::{
        acceleration, angular_velocity, area::square_inch, length, mass::pound, my_quantity,
        thermodynamic_temperature::degree_celsius as celsius, typenum::P2, velocity, Acceleration,
        Angle, AngularVelocity, Area, ArealMassDensity, ConstZero, Length, Mass, MassDensity,
        MolarHeatCapacity, MolarMass, Pressure, Ratio, ReciprocalLength, Velocity,
    },
    vectors::{Cross, MyVector3, Norm},
    Numeric,
};

pub trait DragFunction {
    fn cd(&self, velocity: Velocity) -> ReciprocalLength;
}

pub trait DragInit: Sized {
    fn new(simulation: &Simulation<Self>) -> Self;
}

// Drag
impl<D> Simulation<D>
where
    D: DragFunction,
{
    // Initial work to predetermine terminal velocity - not sure how to determine which value to use for
    // cd without solving ODE
    // pub fn terminal_velocity(&self) -> Velocity {
    //     let icd = self.projectile.bc() / (D::cd(0.562).expect("CD") * self.atmosphere.rho());

    //     (self.shooter.gravity() * icd).norm().sqrt()
    // }

    // Drag acceleration vector
    // The velocity used is projectile.velocity - wind.velocity because wind will have a negative impact
    // on drag force - it's not from the projectile "being blown" by the wind
    // This is also why this adjust velocity is only used here, and not saved as delta during iteration
    // Drag force is proportional to square of velocity and area of projectile, scaled
    // by a coefficient at mach speeds (approximately)
    // Optimization: Mass/Area do not impact function, they cancel out and leave factor of FRAC_PI_4
    // which can be further reduced to FRAC_PI_8 (due to the multiplication by -0.5)
    // -FRAC_PI_8 can be inlined into table at compile time

    // a = -0.5 * cd * rho * V * v * area * i * 1/m
    // i = sd/bc
    // sd = m/d^2
    // i = m/d^2 * (1/bc)

    // area = ((1/2) * d)^2 * pi
    // area = (1/4) * d^2 * pi
    // area = pi/4 * d^2

    // a = -0.5 * cd * rho * V * v * pi/4 * d^2 * i * 1/m
    // a = -0.5 * cd * rho * V * v * pi/4 * d^2 * m/d^2 * 1/bc * 1/m
    // a = -pi/8 * cd(v) * V * v * rho * (1/bc)
    // this means constants can be moved and multipled into "y's" of drag table

    // FAST: a = V * v * cd'(v) * rho * 1/bc
    // SLOW: a = V * v * cd(v) * rho * area * i * 1/m * -0.5
    pub(crate) fn drag_acceleration(
        &self,
        velocity: MyVector3<velocity::Dimension>,
    ) -> MyVector3<acceleration::Dimension> {
        if self.flags.drag {
            let velocity = velocity - self.wind_velocity();
            let norm = velocity.norm();
            let cd = self.drag.as_ref().unwrap().cd(norm);
            velocity * norm * cd
        } else {
            MyVector3::ZERO
        }
    }

    pub(crate) fn acceleration(
        &self,
        velocity: MyVector3<velocity::Dimension>,
    ) -> MyVector3<acceleration::Dimension> {
        self.coriolis_acceleration(velocity)
            + self.drag_acceleration(velocity)
            + self.gravity_acceleration()
    }
}

impl<D> Simulation<D> {
    pub fn sound_velocity(&self) -> Velocity {
        self.atmosphere.sound_velocity()
    }

    // Velocity relative to speed of sound, with given atmospheric conditions
    pub fn mach(&self, velocity: Velocity) -> Ratio {
        velocity / self.sound_velocity()
    }

    // Velocity vector of wind, only horizontal at the moment
    // Does not adjust according to line of sight, since most would measure wind
    // along relative bearing - I don't think many would factor in a 'downhill' wind for example
    // This would be interresting to think of, however.
    pub(crate) fn wind_velocity(&self) -> MyVector3<velocity::Dimension> {
        self.atmosphere
            .wind
            .velocity()
            .pivot_x(self.shooter.roll())
            .pivot_z(self.shooter.pitch())
            .pivot_y(self.shooter.yaw())
    }

    // Projectiles initial velocity relative to scope
    pub(crate) fn velocity(&self) -> MyVector3<velocity::Dimension> {
        MyVector3::new(self.projectile.velocity, Velocity::ZERO, Velocity::ZERO)
            .pivot_y(self.scope.yaw())
            .pivot_z(self.scope.pitch())
            .pivot_x(self.shooter.roll())
            .pivot_z(self.shooter.pitch())
            .pivot_y(self.shooter.yaw())
    }

    // Projectiles initial position relative to scope
    pub(crate) fn position(&self) -> MyVector3<length::Dimension> {
        MyVector3::new(Length::ZERO, -self.scope.height, -self.scope.offset)
            .pivot_x(self.scope.roll())
            .pivot_x(self.shooter.roll())
            .pivot_z(self.shooter.pitch())
            .pivot_y(self.shooter.yaw())
    }

    // Coriolis/Eotovos acceleration vector.
    // Accounts for Left/Right drift due to Earth's spin
    // This drift is always right (+z relative) in the northern hemisphere, regardless of initial bearing
    // This drive is always left (-z relative) in the southern hemisphere, regardless of initial bearing
    // Also accounts for elevation changes when launching projectils East/West, regardless of hemisphere
    // Bearing East results in higher elevation (+y absolute/relative)
    // Bearing West results in lower elevation (-y relative/absolute)
    pub(crate) fn coriolis_acceleration(
        &self,
        velocity: MyVector3<velocity::Dimension>,
    ) -> MyVector3<acceleration::Dimension> {
        if self.flags.coriolis {
            self.shooter.omega().cross(&velocity) * -2.0
        } else {
            MyVector3::ZERO
        }
    }

    // Gravity acceleration vector
    pub(crate) fn gravity_acceleration(&self) -> MyVector3<acceleration::Dimension> {
        if self.flags.gravity {
            self.shooter.gravity()
        } else {
            MyVector3::ZERO
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
    pub fn rho(&self) -> MassDensity {
        ((self.pd() * Self::MOLAR_MASS_DRY_AIR) + (self.pv() * Self::MOLAR_MASS_WATER_VAPOR))
            / (Self::MOLAR_GAS_UNIVERSAL * self.temperature)
    }

    // Speed of sound at given air density and pressure
    pub fn sound_velocity(&self) -> Velocity {
        (Self::ADIABATIC_INDEX_AIR * (self.pressure / self.rho())).sqrt()
    }

    // Pressure of water vapor, Arden Buck equation
    fn pv(&self) -> Pressure {
        my_quantity!(
            self.humidity
                * 611.21
                * ((18.678 - (self.celsius() / 234.5))
                    * (self.celsius() / (257.14 + self.celsius())))
                .exp()
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

impl Scope {
    pub(crate) fn yaw(&self) -> Angle {
        -self.yaw
    }

    pub(crate) fn pitch(&self) -> Angle {
        self.pitch
    }

    pub(crate) fn roll(&self) -> Angle {
        self.roll
    }
}

impl Shooter {
    // Angular velocity of earth, (rad/s)
    const ANGULAR_VELOCITY: AngularVelocity = my_quantity!(0.000_072_921_159);

    // Gravity of earth (m/s^2)
    const GRAVITY: Acceleration = my_quantity!(-9.806_65);

    pub fn gravity(&self) -> MyVector3<acceleration::Dimension> {
        MyVector3::new(Acceleration::ZERO, Self::GRAVITY, Acceleration::ZERO)
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
    pub fn omega(&self) -> MyVector3<angular_velocity::Dimension> {
        MyVector3::new(
            Self::ANGULAR_VELOCITY,
            AngularVelocity::ZERO,
            AngularVelocity::ZERO,
        )
        .pivot_z(self.latitude)
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
        MyVector3::new(self.velocity, Velocity::ZERO, Velocity::ZERO)
            .pivot_y(self.yaw())
            .pivot_z(self.pitch())
            .pivot_x(self.roll())
    }
}

impl Projectile {
    pub fn area(&self) -> Area {
        PI * self.radius().powi(P2::new())
    }

    pub fn i(&self) -> Ratio {
        self.sd() / self.bc()
    }

    pub fn radius(&self) -> Length {
        self.caliber / 2.0
    }

    pub fn bc(&self) -> ArealMassDensity {
        let mass = Mass::new::<pound>(self.bc);
        let area = Area::new::<square_inch>(1.0);
        mass / area
    }

    pub fn sd(&self) -> ArealMassDensity {
        self.weight / self.caliber.powi(P2::new())
    }
}
