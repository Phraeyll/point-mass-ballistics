use crate::{
    consts::{FRAC_PI_2, PI},
    error::{Error, Result},
    units::{
        celsius, kelvin, kilogram, meter, meter_per_second, pascal, radian, second, Angle,
        ConstZero, Length, Mass, Pressure, ThermodynamicTemperature, Time, Velocity,
    },
    Numeric,
};

use std::marker::PhantomData;

#[derive(Debug)]
pub struct Simulation<D> {
    _marker: PhantomData<D>,
    pub(crate) flags: Flags, // Flags to enable/disable certain parts of simulation
    pub(crate) projectile: Projectile, // Use same projectile for zeroing and solving
    pub(crate) scope: Scope, // Use same scope for zeroing and solving
    pub(crate) atmosphere: Atmosphere, // Different conditions during solving
    pub(crate) wind: Wind,   // Different conditions during solving
    pub(crate) shooter: Shooter, // Different conditions during solving
    pub(crate) time_step: Time, // Use same timestep for zeroing and solving
}

#[derive(Debug)]
pub struct Atmosphere {
    pub(crate) temperature: ThermodynamicTemperature, // Temperature (F)
    pub(crate) pressure: Pressure,                    // Pressure (InHg)
    pub(crate) humidity: Numeric,                     // Humidity (0-1)
}

#[derive(Debug)]
pub struct Flags {
    pub(crate) coriolis: bool, // Whether or not to calculate coriolis/eotvos effect
    pub(crate) drag: bool,     // Whether or not to calculate drag
    pub(crate) gravity: bool,  // Whether or not to calculate gravity
}

#[derive(Debug)]
pub struct Projectile {
    pub caliber: Length,
    pub weight: Mass,
    pub bc: Numeric,
    pub velocity: Velocity,
}

#[derive(Debug)]
pub struct Scope {
    pub(crate) yaw: Angle,
    pub(crate) pitch: Angle,
    pub(crate) roll: Angle,    // Scope Roll (Cant) (Degrees)
    pub(crate) height: Length, // Scope Height (inches)
    pub(crate) offset: Length, // Scope Offset Windage (left/right boreline) (inches)
}

#[derive(Debug)]
pub struct Shooter {
    pub(crate) yaw: Angle, // Bearing (0 North, 90 East) (degrees) (Coriolis/Eotvos Effect)
    pub(crate) pitch: Angle, // Line of Sight angle (degrees)
    pub(crate) roll: Angle, // Roll relative to shooters position, ie, scope alligned with rifle
    pub(crate) lattitude: Angle, // Lattitude (Coriolis/Eotvos Effect)
}

#[derive(Debug)]
pub struct Wind {
    pub(crate) yaw: Angle,         // Wind Angle (degrees)
    pub(crate) pitch: Angle,       // Wind Pitch (degrees)
    pub(crate) roll: Angle,        // Doesn make sense, just here for consistency
    pub(crate) velocity: Velocity, // Wind Velocity (miles/hour)
}

#[derive(Debug)]
pub struct SimulationBuilder<D> {
    pub(crate) builder: Simulation<D>,
}

impl<D> From<SimulationBuilder<D>> for Simulation<D> {
    fn from(other: SimulationBuilder<D>) -> Self {
        Self { ..other.builder }
    }
}

impl<D> From<Simulation<D>> for SimulationBuilder<D> {
    fn from(other: Simulation<D>) -> Self {
        Self { builder: other }
    }
}

impl<D> Default for SimulationBuilder<D> {
    fn default() -> Self {
        Self {
            builder: Simulation {
                _marker: PhantomData,
                flags: Flags {
                    coriolis: true,
                    drag: true,
                    gravity: true,
                },
                projectile: Projectile {
                    caliber: Length::ZERO,
                    weight: Mass::ZERO,
                    bc: 0.0,
                    velocity: Velocity::ZERO,
                },
                scope: Scope {
                    yaw: Angle::ZERO,
                    pitch: Angle::ZERO,
                    roll: Angle::ZERO,
                    height: Length::ZERO,
                    offset: Length::ZERO,
                },
                atmosphere: Atmosphere {
                    temperature: ThermodynamicTemperature::ZERO,
                    pressure: Pressure::ZERO,
                    humidity: 0.0,
                },
                wind: Wind {
                    yaw: Angle::ZERO,
                    pitch: Angle::ZERO,
                    roll: Angle::ZERO,
                    velocity: Velocity::ZERO,
                },
                shooter: Shooter {
                    yaw: Angle::ZERO,
                    pitch: Angle::ZERO,
                    roll: Angle::ZERO,
                    lattitude: Angle::ZERO,
                },
                time_step: Time::ZERO,
            },
        }
    }
}

impl<D> SimulationBuilder<D> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<D> SimulationBuilder<D> {
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    pub fn init(self) -> Simulation<D> {
        From::from(self)
    }

    pub fn set_time_step(mut self, value: Time) -> Result<Self> {
        let min = Time::new::<second>(0.0);
        let max = Time::new::<second>(0.1);
        if value > min && value <= max {
            self.builder.time_step = value;
            Ok(self)
        } else {
            Err(Error::OutOfRange {
                min: min.get::<second>(),
                max: max.get::<second>(),
            })
        }
    }

    // Atmosphere
    pub fn set_temperature(mut self, value: ThermodynamicTemperature) -> Result<Self> {
        let min = ThermodynamicTemperature::new::<celsius>(-80.0);
        let max = ThermodynamicTemperature::new::<celsius>(50.0);
        if value >= min && value <= max {
            self.builder.atmosphere.temperature = value;
            Ok(self)
        } else {
            Err(Error::OutOfRange {
                min: min.get::<kelvin>(),
                max: max.get::<kelvin>(),
            })
        }
    }

    pub fn set_pressure(mut self, value: Pressure) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.atmosphere.pressure = value;
            Ok(self)
        } else {
            Err(Error::PositiveExpected(value.get::<pascal>()))
        }
    }

    pub fn set_humidity(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (0.0, 1.0);
        if value >= min && value <= max {
            self.builder.atmosphere.humidity = value;
            Ok(self)
        } else {
            Err(Error::OutOfRange { min, max })
        }
    }

    // Flags
    pub fn use_coriolis(mut self, value: bool) -> Self {
        self.builder.flags.coriolis = value;
        self
    }

    pub fn use_drag(mut self, value: bool) -> Self {
        self.builder.flags.drag = value;
        self
    }

    pub fn use_gravity(mut self, value: bool) -> Self {
        self.builder.flags.gravity = value;
        self
    }

    // Shooter
    pub fn set_incline(mut self, value: Angle) -> Result<Self> {
        let min = Angle::new::<radian>(-FRAC_PI_2);
        let max = Angle::new::<radian>(FRAC_PI_2);
        if value >= min && value <= max {
            self.builder.shooter.pitch = value;
            Ok(self)
        } else {
            Err(Error::OutOfRange {
                min: min.get::<radian>(),
                max: max.get::<radian>(),
            })
        }
    }

    pub fn set_lattitude(mut self, value: Angle) -> Result<Self> {
        let min = Angle::new::<radian>(-FRAC_PI_2);
        let max = Angle::new::<radian>(FRAC_PI_2);
        if value >= min && value <= max {
            self.builder.shooter.lattitude = value;
            Ok(self)
        } else {
            Err(Error::OutOfRange {
                min: min.get::<radian>(),
                max: max.get::<radian>(),
            })
        }
    }

    pub fn set_bearing(mut self, value: Angle) -> Result<Self> {
        let min = Angle::new::<radian>(-2.0 * PI);
        let max = Angle::new::<radian>(2.0 * PI);
        if value >= min && value <= max {
            self.builder.shooter.yaw = value;
            Ok(self)
        } else {
            Err(Error::OutOfRange {
                min: min.get::<radian>(),
                max: max.get::<radian>(),
            })
        }
    }

    // Wind
    pub fn set_wind_speed(mut self, value: Velocity) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.wind.velocity = value;
            Ok(self)
        } else {
            Err(Error::PositiveExpected(value.get::<meter_per_second>()))
        }
    }

    pub fn set_wind_direction(mut self, value: Angle) -> Result<Self> {
        let min = Angle::new::<radian>(-2.0 * PI);
        let max = Angle::new::<radian>(2.0 * PI);
        if value >= min && value <= max {
            self.builder.wind.yaw = value;
            Ok(self)
        } else {
            Err(Error::OutOfRange {
                min: min.get::<radian>(),
                max: max.get::<radian>(),
            })
        }
    }

    //Scope
    pub fn set_scope_height(mut self, value: Length) -> Self {
        self.builder.scope.height = value;
        self
    }

    pub fn set_scope_offset(mut self, value: Length) -> Self {
        self.builder.scope.offset = value;
        self
    }

    pub fn set_scope_pitch(mut self, value: Angle) -> Self {
        self.builder.scope.pitch = value;
        self
    }

    pub fn set_scope_yaw(mut self, value: Angle) -> Self {
        self.builder.scope.yaw = value;
        self
    }

    pub fn set_scope_roll(mut self, value: Angle) -> Self {
        self.builder.scope.roll = value;
        self
    }

    //Projectile
    pub fn set_caliber(mut self, value: Length) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.projectile.caliber = value;
            Ok(self)
        } else {
            Err(Error::PositiveExpected(value.get::<meter>()))
        }
    }

    pub fn set_velocity(mut self, value: Velocity) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.projectile.velocity = value;
            Ok(self)
        } else {
            Err(Error::PositiveExpected(value.get::<meter_per_second>()))
        }
    }

    pub fn set_mass(mut self, value: Mass) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.projectile.weight = value;
            Ok(self)
        } else {
            Err(Error::PositiveExpected(value.get::<kilogram>()))
        }
    }

    pub fn set_bc(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.projectile.bc = value;
            Ok(self)
        } else {
            Err(Error::PositiveExpected(value))
        }
    }
}
