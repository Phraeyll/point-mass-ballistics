use nalgebra::Vector3;

use crate::util::*;
use crate::model::core::{Wind, Atmosphere, Shooter, Flags, Projectile, Scope, SimulationBuilder};

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

impl Simulation {
    // Rotated velocity vector, accounts for muzzle/shooter pitch, and yaw (bearing)
    // Start with velocity value along X unit vector
    pub(crate) fn absolute_projectile_velocity(&self) -> Vector3<Numeric> {
        self.projectile
            .velocity(&self.scope)
            .pivot_z(self.shooter.line_of_sight)
            .pivot_y(self.shooter.corrected_azimuth())
    }
    // Projectiles position relative to scope
    pub(crate) fn absolute_projectile_position(&self) -> Vector3<Numeric> {
        -self.scope.position().pivot_x(self.scope.roll)
    }
    // Velocity vector of wind, only horizontal at the moment
    // Does not adjust according to line of sight, since most would measure wind
    // along relative bearing - I don't think many would factor in a 'downhill' wind for example
    // This would be interresting to think of, however.
    pub(crate) fn absolute_wind_velocity(&self) -> Vector3<Numeric> {
        self
            .wind
            .velocity()
            .pivot_y(self.shooter.corrected_azimuth())
    }
    pub fn increment_scope_pitch(&mut self, value: Numeric) {
        self.scope.pitch += Angle::Minutes(value);
    }
    pub fn increment_scope_yaw(&mut self, value: Numeric) {
        self.scope.yaw += Angle::Minutes(value);
    }
}
