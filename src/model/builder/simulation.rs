use crate::model::core::{Angles, Conditions, Flags, Projectile, Scope, Simulation};
use crate::util::*;

pub struct SimulationBuilder {
    pub flags: Flags,           // Flags to enable/disable certain parts of simulation
    pub projectile: Projectile, // Use same projectile for zeroing and solving
    pub scope: Scope,           // Use same scope for zeroing and solving
    pub conditions: Conditions, // Different conditions during solving
    pub time_step: Time,        // Use same timestep for zeroing and solving
}
impl From<Simulation> for SimulationBuilder {
    fn from(other: Simulation) -> Self {
        Self {
            flags: other.flags,
            projectile: other.projectile,
            scope: other.scope,
            conditions: other.conditions,
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
            time_step: Time::Seconds(0.000_001),
        }
    }
}

pub trait Builder {
    type Simulation;

    fn new() -> Self;
    fn projectile(self, value: Projectile) -> Self;
    fn scope(self, value: Scope) -> Self;
    fn conditions(self, value: Conditions) -> Self;
    fn time_step(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn flags(self, value: Flags) -> Self;
    fn create_with(self, angles: Angles) -> Self::Simulation;
}
impl Builder for SimulationBuilder {
    type Simulation = Simulation;
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    fn create_with(self, angles: Angles) -> Self::Simulation {
        Simulation::new(
         self.flags,
         self.projectile,
         self.scope,
         self.conditions,
            angles,
            self.time_step,
        )
    }
    // Create a simulation with muzzle pitch found in 'zeroin' simulation
    // Then solve for current conditions
    // Can be used for drop table, or eventually dialing in a specific distance
    fn new() -> Self {
        Self::default()
    }
    fn flags(mut self, value: Flags) -> Self {
        self.flags = value;
        self
    }
    fn projectile(mut self, value: Projectile) -> Self {
        self.projectile = value;
        self
    }
    fn scope(mut self, value: Scope) -> Self {
        self.scope = value;
        self
    }
    fn conditions(mut self, value: Conditions) -> Self {
        self.conditions = value;
        self
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
