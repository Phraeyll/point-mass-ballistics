use crate::model::core::{BcKind, Simulation, SimulationBuilder};
use crate::util::*;

impl Builder for SimulationBuilder {
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    type Simulation = Simulation;
    fn init(self) -> Result<Self::Simulation> {
        match self.projectile.bc.kind {
            BcKind::Null => Err(Error::new(ErrorKind::BcKindNull)),
            _ => Ok(Self::Simulation::from(self)),
        }
    }
    fn time_step(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (0.0, 0.1);
        if value > min && value <= max {
            self.time_step = value;
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
        Self::default()
    }
    fn init(self) -> Result<Self::Simulation>
    where
        Self: Sized;
    fn time_step(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
}

pub trait ScopeAdjuster {
    fn set_scope_height(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_scope_offset(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_scope_pitch(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_scope_yaw(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn set_scope_roll(self, value: Numeric) -> Result<Self>
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
