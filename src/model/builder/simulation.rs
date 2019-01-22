use crate::model::core::{Angles, Bc, BcKind, Conditions, Flags, Projectile, Scope, Simulation};
use crate::util::*;

pub struct SimulationBuilder {
    pub flags: Flags,           // Flags to enable/disable certain parts of simulation
    pub projectile: Projectile, // Use same projectile for zeroing and solving
    pub scope: Scope,           // Use same scope for zeroing and solving
    pub conditions: Conditions, // Different conditions during solving
    pub angles: Angles,         // Use same timestep for zeroing and solving
    pub time_step: Time,        // Use same timestep for zeroing and solving
}

impl From<Simulation> for SimulationBuilder {
    fn from(other: Simulation) -> Self {
        Self {
            flags: other.flags,
            projectile: other.projectile,
            scope: other.scope,
            conditions: other.conditions,
            angles: other.angles,
            time_step: other.time_step,
        }
    }
}

impl Default for SimulationBuilder {
    fn default() -> Self {
        Self {
            flags: Flags::default(),
            projectile: Projectile::default(),
            scope: Scope::default(),
            conditions: Conditions::default(),
            angles: Angles::default(),
            time_step: Time::Seconds(0.000_001),
        }
    }
}

impl Builder for SimulationBuilder {
    type Simulation = Simulation;
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    fn init_with(mut self, value: Bc) -> Self::Simulation {
        self.projectile.bc = value;
        Simulation::from(self)
    }
    fn new() -> Self {
        Self::default()
    }
    fn time_step(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (0.0, 0.1);
        if value > min && value <= max {
            self.time_step = Time::Seconds(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
}

pub trait Builder {
    type Simulation;
    // Creation and Finalization
    fn new() -> Self;
    fn init_with(self, value: Bc) -> Self::Simulation;

    // timestep
    fn time_step(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
}

pub trait BcBuilder {
    fn with(value: Numeric, kind: BcKind) -> Result<Self>
    where
        Self: Sized;
}

pub trait ScopeBuilder {
    fn set_height(self, value: Numeric) -> Self;
    fn set_offset(self, value: Numeric) -> Self;
}

pub trait ProjectileBuilder {
    fn set_velocity(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_grains(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_caliber(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_bc(self, value: Bc) -> Self;
}

pub trait AtmosphereBuilder {
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
pub trait WindBuilder {
    fn set_wind_speed(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_wind_angle(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
}
pub trait OtherBuilder {
    fn set_shot_angle(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_lattitude(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_bearing(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_gravity(self, value: Numeric) -> Self;
}

pub trait FlagsBuilder {
    fn use_coriolis(self, value: bool) -> Self;
    fn use_drag(self, value: bool) -> Self;
    fn use_gravity(self, value: bool) -> Self;
}

pub trait AnglesBuilder {
    // Angles
    fn set_pitch(self, value: Numeric) -> Self;
    fn set_yaw(self, value: Numeric) -> Self;
}
