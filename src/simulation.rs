use crate::{
    consts::{FRAC_PI_2, PI},
    error::{Error, Result},
    physics::DragInit,
    units::{
        angle::radian, length::meter, mass::kilogram, pressure::pascal,
        thermodynamic_temperature::degree_celsius, time::second, velocity::meter_per_second, Angle,
        ConstZero, Length, Mass, Pressure, ThermodynamicTemperature, Time, Velocity,
    },
    Numeric,
};

#[derive(Debug)]
pub struct Simulation<D> {
    pub(crate) drag: Option<D>,
    pub(crate) flags: Flags,
    pub(crate) projectile: Projectile,
    pub(crate) scope: Scope,
    pub(crate) atmosphere: Atmosphere,
    pub(crate) shooter: Shooter,
    pub(crate) time_step: Time,
}

#[derive(Debug)]
pub struct Atmosphere {
    pub(crate) temperature: ThermodynamicTemperature,
    pub(crate) pressure: Pressure,
    pub(crate) humidity: Numeric,
    pub(crate) wind: Wind,
}

#[derive(Debug)]
pub struct Flags {
    pub(crate) coriolis: bool,
    pub(crate) drag: bool,
    pub(crate) gravity: bool,
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
    pub(crate) roll: Angle,
    pub(crate) height: Length,
    pub(crate) offset: Length,
}

#[derive(Debug)]
pub struct Shooter {
    pub(crate) yaw: Angle,
    pub(crate) pitch: Angle,
    pub(crate) roll: Angle,
    pub(crate) latitude: Angle,
}

#[derive(Debug)]
pub struct Wind {
    pub(crate) yaw: Angle,
    pub(crate) pitch: Angle,
    pub(crate) velocity: Velocity,
}

#[derive(Debug)]
pub struct SimulationBuilder<D>(Simulation<D>);

impl<D> Default for SimulationBuilder<D> {
    fn default() -> Self {
        Self(Simulation {
            drag: None,
            flags: Flags {
                coriolis: true,
                drag: true,
                gravity: true,
            },
            projectile: Projectile {
                caliber: Length::ZERO,
                weight: Mass::ZERO,
                bc: Numeric::ZERO,
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
                humidity: Numeric::ZERO,
                wind: Wind {
                    yaw: Angle::ZERO,
                    pitch: Angle::ZERO,
                    velocity: Velocity::ZERO,
                },
            },
            shooter: Shooter {
                yaw: Angle::ZERO,
                pitch: Angle::ZERO,
                roll: Angle::ZERO,
                latitude: Angle::ZERO,
            },
            time_step: Time::ZERO,
        })
    }
}

impl<D> SimulationBuilder<D>
where
    D: DragInit,
{
    pub fn init(mut self) -> Simulation<D> {
        self.0.drag = Some(D::new(&self.0));
        self.0
    }
}

impl<D> SimulationBuilder<D> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_time_step(mut self, value: Time) -> Result<Self> {
        let min = Time::ZERO;
        let max = Time::new::<second>(0.1);
        if value > min && value <= max {
            self.0.time_step = value;
            Ok(self)
        } else {
            Err(Error::TimeOutOfRange { value, min, max })
        }
    }

    // Atmosphere
    pub fn set_temperature(mut self, value: ThermodynamicTemperature) -> Result<Self> {
        let min = ThermodynamicTemperature::new::<degree_celsius>(-80.0);
        let max = ThermodynamicTemperature::new::<degree_celsius>(50.0);
        if value >= min && value <= max {
            self.0.atmosphere.temperature = value;
            Ok(self)
        } else {
            Err(Error::ThermodynamicTemperatureOutOfRange { value, min, max })
        }
    }

    pub fn set_pressure(mut self, value: Pressure) -> Result<Self> {
        if value.is_sign_positive() {
            self.0.atmosphere.pressure = value;
            Ok(self)
        } else {
            Err(Error::PositiveExpected {
                value: value.get::<pascal>(),
            })
        }
    }

    pub fn set_humidity(mut self, value: Numeric) -> Result<Self> {
        let min = 0.0;
        let max = 1.0;
        if value >= min && value <= max {
            self.0.atmosphere.humidity = value;
            Ok(self)
        } else {
            Err(Error::NumericOutOfRange { value, min, max })
        }
    }

    // Flags
    pub fn use_coriolis(mut self, value: bool) -> Self {
        self.0.flags.coriolis = value;
        self
    }

    pub fn use_drag(mut self, value: bool) -> Self {
        self.0.flags.drag = value;
        self
    }

    pub fn use_gravity(mut self, value: bool) -> Self {
        self.0.flags.gravity = value;
        self
    }

    // Shooter
    pub fn set_incline(mut self, value: Angle) -> Result<Self> {
        let min = Angle::new::<radian>(-FRAC_PI_2);
        let max = Angle::new::<radian>(FRAC_PI_2);
        if value >= min && value <= max {
            self.0.shooter.pitch = value;
            Ok(self)
        } else {
            Err(Error::AngleOutOfRange { value, min, max })
        }
    }

    pub fn set_latitude(mut self, value: Angle) -> Result<Self> {
        let min = Angle::new::<radian>(-FRAC_PI_2);
        let max = Angle::new::<radian>(FRAC_PI_2);
        if value >= min && value <= max {
            self.0.shooter.latitude = value;
            Ok(self)
        } else {
            Err(Error::AngleOutOfRange { value, min, max })
        }
    }

    pub fn set_bearing(mut self, value: Angle) -> Result<Self> {
        let min = Angle::new::<radian>(-2.0 * PI);
        let max = Angle::new::<radian>(2.0 * PI);
        if value >= min && value <= max {
            self.0.shooter.yaw = value;
            Ok(self)
        } else {
            Err(Error::AngleOutOfRange { value, min, max })
        }
    }

    // Wind
    pub fn set_wind_speed(mut self, value: Velocity) -> Result<Self> {
        if value.is_sign_positive() {
            self.0.atmosphere.wind.velocity = value;
            Ok(self)
        } else {
            Err(Error::PositiveExpected {
                value: value.get::<meter_per_second>(),
            })
        }
    }

    pub fn set_wind_direction(mut self, value: Angle) -> Result<Self> {
        let min = Angle::new::<radian>(-2.0 * PI);
        let max = Angle::new::<radian>(2.0 * PI);
        if value >= min && value <= max {
            self.0.atmosphere.wind.yaw = value;
            Ok(self)
        } else {
            Err(Error::AngleOutOfRange { value, min, max })
        }
    }

    //Scope
    pub fn set_scope_height(mut self, value: Length) -> Self {
        self.0.scope.height = value;
        self
    }

    pub fn set_scope_offset(mut self, value: Length) -> Self {
        self.0.scope.offset = value;
        self
    }

    pub fn set_scope_pitch(mut self, value: Angle) -> Self {
        self.0.scope.pitch = value;
        self
    }

    pub fn set_scope_yaw(mut self, value: Angle) -> Self {
        self.0.scope.yaw = value;
        self
    }

    pub fn set_scope_roll(mut self, value: Angle) -> Self {
        self.0.scope.roll = value;
        self
    }

    //Projectile
    pub fn set_caliber(mut self, value: Length) -> Result<Self> {
        if value.is_sign_positive() {
            self.0.projectile.caliber = value;
            Ok(self)
        } else {
            Err(Error::PositiveExpected {
                value: value.get::<meter>(),
            })
        }
    }

    pub fn set_velocity(mut self, value: Velocity) -> Result<Self> {
        if value.is_sign_positive() {
            self.0.projectile.velocity = value;
            Ok(self)
        } else {
            Err(Error::PositiveExpected {
                value: value.get::<meter_per_second>(),
            })
        }
    }

    pub fn set_mass(mut self, value: Mass) -> Result<Self> {
        if value.is_sign_positive() {
            self.0.projectile.weight = value;
            Ok(self)
        } else {
            Err(Error::PositiveExpected {
                value: value.get::<kilogram>(),
            })
        }
    }

    pub fn set_bc(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.0.projectile.bc = value;
            Ok(self)
        } else {
            Err(Error::PositiveExpected { value: value })
        }
    }
}
