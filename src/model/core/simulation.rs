use nalgebra::Vector3;

use crate::model::core::{Atmosphere, Flags, Projectile, Scope, Shooter, Wind, AtmosphereBuilder, FlagsBuilder, ProjectileBuilder, ScopeBuilder, ShooterBuilder, WindBuilder};
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
#[derive(Debug)]
pub struct SimulationBuilder {
    pub flags: FlagsBuilder, // Flags to enable/disable certain parts of simulation
    pub projectile: ProjectileBuilder, // Use same projectile for zeroing and solving
    pub scope: ScopeBuilder, // Use same scope for zeroing and solving
    pub atmosphere: AtmosphereBuilder, // Different conditions during solving
    pub wind: WindBuilder,   // Different conditions during solving
    pub shooter: ShooterBuilder, // Different conditions during solving
    pub time_step: Numeric, // Use same timestep for zeroing and solving
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

impl Simulation {
    //TODO: I think this actually does need to be euler angles now, look into

    // Rotated velocity vector, accounts for muzzle/shooter pitch, and yaw (bearing)
    // Start with velocity value along X unit vector
    pub(crate) fn absolute_projectile_velocity(&self) -> Vector3<Numeric> {
        self.projectile
            .velocity(&self.scope)
            .pivot_x(self.shooter.roll())
            .pivot_z(self.shooter.pitch())
            .pivot_y(self.shooter.yaw())
    }
    // Projectiles position relative to scope
    pub(crate) fn absolute_projectile_position(&self) -> Vector3<Numeric> {
        -self
            .scope
            .position()
            .pivot_x(self.shooter.roll())
            .pivot_x(-self.scope.roll())
            .pivot_z(self.shooter.pitch())
            .pivot_y(self.shooter.yaw())
    }
    // Velocity vector of wind, only horizontal at the moment
    // Does not adjust according to line of sight, since most would measure wind
    // along relative bearing - I don't think many would factor in a 'downhill' wind for example
    // This would be interresting to think of, however.
    pub(crate) fn absolute_wind_velocity(&self) -> Vector3<Numeric> {
        self.wind
            .velocity()
            .pivot_x(self.shooter.roll())
            .pivot_z(self.shooter.pitch())
            .pivot_y(self.shooter.yaw())
    }
    pub fn increment_scope_pitch(&mut self, value: Numeric) {
        self.scope.pitch += Angle::Minutes(value);
    }
    pub fn increment_scope_yaw(&mut self, value: Numeric) {
        self.scope.yaw += Angle::Minutes(value);
    }
}
