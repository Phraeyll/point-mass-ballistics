use crate::model::core::{
    Atmosphere, AtmosphereBuilder, Flags, FlagsBuilder, Projectile, ProjectileBuilder, Scope,
    ScopeBuilder, Shooter, ShooterBuilder, Wind, WindBuilder,
};
use crate::util::*;

#[derive(Debug)]
pub struct Simulation {
    pub(crate) flags: Flags,
    pub(crate) projectile: Projectile,
    pub(crate) scope: Scope,
    pub(crate) atmosphere: Atmosphere,
    pub(crate) wind: Wind,
    pub(crate) shooter: Shooter,
    pub(crate) time_step: Numeric,
}
#[derive(Debug)]
pub struct SimulationBuilder {
    pub flags: FlagsBuilder, // Flags to enable/disable certain parts of simulation
    pub projectile: ProjectileBuilder, // Use same projectile for zeroing and solving
    pub scope: ScopeBuilder, // Use same scope for zeroing and solving
    pub atmosphere: AtmosphereBuilder, // Different conditions during solving
    pub wind: WindBuilder,   // Different conditions during solving
    pub shooter: ShooterBuilder, // Different conditions during solving
    pub time_step: Numeric,  // Use same timestep for zeroing and solving
}
impl From<SimulationBuilder> for Simulation {
    fn from(other: SimulationBuilder) -> Self {
        Self {
            flags: Flags::from(other.flags),
            projectile: Projectile::from(other.projectile),
            scope: Scope::from(other.scope),
            atmosphere: Atmosphere::from(other.atmosphere),
            wind: Wind::from(other.wind),
            shooter: Shooter::from(other.shooter),
            time_step: other.time_step,
        }
    }
}
impl From<Simulation> for SimulationBuilder {
    fn from(other: Simulation) -> Self {
        Self {
            flags: FlagsBuilder::from(other.flags),
            projectile: ProjectileBuilder::from(other.projectile),
            scope: ScopeBuilder::from(other.scope),
            atmosphere: AtmosphereBuilder::from(other.atmosphere),
            wind: WindBuilder::from(other.wind),
            shooter: ShooterBuilder::from(other.shooter),
            time_step: other.time_step,
        }
    }
}
impl Default for SimulationBuilder {
    fn default() -> Self {
        Self {
            flags: FlagsBuilder::default(),
            projectile: ProjectileBuilder::default(),
            scope: ScopeBuilder::default(),
            atmosphere: AtmosphereBuilder::default(),
            wind: WindBuilder::default(),
            shooter: ShooterBuilder::default(),
            time_step: 0.000_001,
        }
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
