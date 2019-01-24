use crate::model::core::{
    dragtables::*, Atmosphere, BcKind, BcKind::*, Flags, Projectile, Scope, Shooter, Simulation,
    Wind,
};
use crate::util::*;

#[derive(Debug)]
pub struct SimulationBuilder {
    pub flags: Flags,           // Flags to enable/disable certain parts of simulation
    pub projectile: Projectile, // Use same projectile for zeroing and solving
    pub scope: Scope,           // Use same scope for zeroing and solving
    pub atmosphere: Atmosphere, // Different conditions during solving
    pub wind: Wind,             // Different conditions during solving
    pub shooter: Shooter,       // Different conditions during solving
    pub time_step: Numeric,     // Use same timestep for zeroing and solving
}

impl From<Simulation> for SimulationBuilder {
    fn from(other: Simulation) -> Self {
        Self {
            flags: other.flags,
            projectile: other.projectile,
            scope: other.scope,
            atmosphere: other.atmosphere,
            wind: other.wind,
            shooter: other.shooter,
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
            atmosphere: Atmosphere::default(),
            wind: Wind::default(),
            shooter: Shooter::default(),
            time_step: 0.000_001,
        }
    }
}

impl Builder for SimulationBuilder {
    type Simulation = Simulation;
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    fn new() -> Self {
        Self::default()
    }
    fn time_step(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (0.0, 0.1);
        if value > min && value <= max {
            self.time_step = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn init_with(mut self, value: Numeric, kind: BcKind) -> Result<Self::Simulation> {
        if value.is_sign_positive() {
            self.projectile.bc.value = value;
            self.projectile.bc.kind = kind;
            self.projectile.bc.table = match kind {
                G1 => g1::init(),
                G2 => g2::init(),
                G5 => g5::init(),
                G6 => g6::init(),
                G7 => g7::init(),
                G8 => g8::init(),
                GI => gi::init(),
                GS => gs::init(),
            };
            Ok(Simulation::from(self))
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
}

pub trait Builder {
    type Simulation;
    fn new() -> Self
    where
        Self: Sized;
    fn time_step(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn init_with(self, value: Numeric, kind: BcKind) -> Result<Self::Simulation>
    where
        Self: Sized;
}

pub trait ScopeBuilder {
    fn set_height(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_offset(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_pitch(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_yaw(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_roll(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
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
pub trait ShooterBuilder {
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

pub trait FlagsBuilder {
    fn use_coriolis(self, value: bool) -> Result<Self>
    where
        Self: Sized;
    fn use_drag(self, value: bool) -> Result<Self>
    where
        Self: Sized;
    fn use_gravity(self, value: bool) -> Result<Self>
    where
        Self: Sized;
}
