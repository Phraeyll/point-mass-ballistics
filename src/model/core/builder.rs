use crate::model::core::{BcKind, Simulation, SimulationBuilder};
use crate::model::core::bc::create_bc;
use crate::util::*;

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
            self.projectile.bc = create_bc(value, kind);
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

pub trait ScopeAdjuster {
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
