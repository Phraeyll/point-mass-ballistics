use crate::{
    model::core::{Atmosphere, Bc, BcKind, Flags, Projectile, Scope, Shooter, Wind},
    util::*,
};

#[derive(Debug)]
pub struct Simulation {
    pub(crate) flags: Flags, // Flags to enable/disable certain parts of simulation
    pub(crate) projectile: Projectile, // Use same projectile for zeroing and solving
    pub(crate) scope: Scope, // Use same scope for zeroing and solving
    pub(crate) atmosphere: Atmosphere, // Different conditions during solving
    pub(crate) wind: Wind,   // Different conditions during solving
    pub(crate) shooter: Shooter, // Different conditions during solving
    pub(crate) time_step: Numeric, // Use same timestep for zeroing and solving
}
#[derive(Debug)]
pub struct SimulationBuilder {
    pub(crate) builder: Simulation,
}
impl From<SimulationBuilder> for Simulation {
    fn from(other: SimulationBuilder) -> Self {
        Self { ..other.builder }
    }
}
impl From<Simulation> for SimulationBuilder {
    fn from(other: Simulation) -> Self {
        Self { builder: other }
    }
}
impl Default for SimulationBuilder {
    fn default() -> Self {
        Self {
            builder: Simulation {
                flags: Flags {
                    coriolis: true,
                    drag: true,
                    gravity: true,
                },
                projectile: Projectile {
                    caliber: Length::Inches(0.264),
                    weight: WeightMass::Grains(140.0),
                    bc: Bc {
                        value: 0.0,
                        kind: BcKind::Null,
                        table: float_map![],
                    },
                    velocity: Velocity::Fps(2710.0),
                },
                scope: Scope {
                    yaw: Angle::Minutes(0.0),
                    pitch: Angle::Minutes(0.0),
                    roll: Angle::Degrees(0.0),
                    height: Length::Inches(1.5),
                    offset: Length::Inches(0.0),
                },
                atmosphere: Atmosphere {
                    temperature: Temperature::F(68.0),
                    pressure: Pressure::Inhg(29.92),
                    humidity: 0.0,
                },
                wind: Wind {
                    yaw: Angle::Degrees(0.0),
                    pitch: Angle::Degrees(0.0),
                    roll: Angle::Degrees(0.0),
                    velocity: Velocity::Mph(0.0),
                },
                shooter: Shooter {
                    yaw: Angle::Minutes(0.0),
                    pitch: Angle::Minutes(0.0),
                    roll: Angle::Degrees(0.0),
                    lattitude: Angle::Degrees(0.0),
                    gravity: Acceleration::Mps2(-9.806_65),
                },
                time_step: 0.000_001,
            },
        }
    }
}

impl Builder for SimulationBuilder {
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    type Simulation = Simulation;
    fn init(self) -> Result<Self::Simulation> {
        match self.builder.projectile.bc.kind {
            BcKind::Null => Err(Error::new(ErrorKind::BcKindNull)),
            _ => Ok(Self::Simulation::from(self)),
        }
    }
    fn time_step(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (0.0, 0.1);
        if value > min && value <= max {
            self.builder.time_step = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
        }
    }
}

pub trait Builder {
    type Simulation;
    fn new() -> Self
    where
        Self: Sized + Default,
    {
        Default::default()
    }
    fn init(self) -> Result<Self::Simulation>
    where
        Self: Sized;
    fn time_step(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
}

pub trait ScopeAdjuster {
    fn set_scope_height(self, value: Numeric) -> Self
    where
        Self: Sized;
    fn set_scope_offset(self, value: Numeric) -> Self
    where
        Self: Sized;
    fn set_scope_pitch(self, value: Numeric) -> Self
    where
        Self: Sized;
    fn set_scope_yaw(self, value: Numeric) -> Self
    where
        Self: Sized;
    fn set_scope_roll(self, value: Numeric) -> Self
    where
        Self: Sized;
}

pub trait ProjectileAdjuster {
    fn set_velocity(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_grains(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_caliber(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_bc(self, value: Numeric, kind: BcKind) -> Result<Self>
    where
        Self: Sized;
}

pub trait AtmosphereAdjuster {
    fn set_temperature(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_pressure(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_humidity(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
}
pub trait WindAdjuster {
    fn set_wind_speed(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_wind_angle(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
}
pub trait ShooterAdjuster {
    fn set_shot_angle(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_lattitude(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_bearing(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_gravity(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
}

pub trait FlagsAdjuster {
    fn use_coriolis(self, value: bool) -> Self
    where
        Self: Sized;
    fn use_drag(self, value: bool) -> Self
    where
        Self: Sized;
    fn use_gravity(self, value: bool) -> Self
    where
        Self: Sized;
}

impl Simulation {
    pub fn air_desnity(&self) -> Numeric {
        Density::Kgpm3(self.atmosphere.rho()).to_lbpf3().to_num()
    }
    pub fn speed_of_sound(&self) -> Numeric {
        Velocity::Mps(self.atmosphere.speed_of_sound())
            .to_fps()
            .to_num()
    }
}
#[derive(Debug)]
pub struct RefSimulation<'a> {
    pub flags: &'a Flags,
    pub projectile: &'a Projectile,
    pub scope: &'a Scope,
    pub atmosphere: &'a Atmosphere,
    pub wind: &'a Wind,
    pub shooter: &'a Shooter,
    pub time_step: Numeric,
}
