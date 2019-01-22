use crate::model::core::Simulation;
use crate::util::*;

pub use angles::*;
pub use conditions::*;
pub use flags::*;
pub use projectile::*;
pub use scope::*;

mod angles;
mod conditions;
mod flags;
mod projectile;
mod scope;

#[allow(clippy::approx_constant)]
pub(crate) mod dragtables {
    pub mod g1;
    pub mod g2;
    pub mod g5;
    pub mod g6;
    pub mod g7;
    pub mod g8;
    pub mod gi;
    pub mod gs;
}

pub struct Builder {
    pub flags: Flags,           // Flags to enable/disable certain parts of simulation
    pub projectile: Projectile, // Use same projectile for zeroing and solving
    pub scope: Scope,           // Use same scope for zeroing and solving
    pub conditions: Conditions, // Different conditions during solving
    pub time_step: Time,        // Use same timestep for zeroing and solving
}
impl Default for Builder {
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

pub trait SimulationBuilder<'a> {
    type Simulation;

    fn new() -> Self;
    fn projectile(self, value: Projectile) -> Self;
    fn scope(self, value: Scope) -> Self;
    fn conditions(self, value: Conditions) -> Self;
    fn time_step(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn flags(self, value: Flags) -> Self;
    fn create_with(&'a self, angles: Angles) -> Self::Simulation;
}
impl<'a> SimulationBuilder<'a> for Builder {
    type Simulation = Simulation<'a>;
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    fn create_with(&'a self, angles: Angles) -> Self::Simulation {
        Simulation::new(
            &self.flags,
            &self.projectile,
            &self.scope,
            &self.conditions,
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
