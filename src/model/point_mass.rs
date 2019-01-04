pub use self::dragtables::BallisticCoefficient;

use nalgebra::Vector3;

use crate::util::{conversions::*, Numeric, nalgebra_helpers::*};

#[allow(clippy::float_cmp)]
pub(crate) mod iter;
pub mod params;

#[allow(clippy::approx_constant)]
mod dragtables;

#[allow(clippy::float_cmp)]
pub(crate) mod zero;

pub(crate) struct Simulation<'c> {
    projectile: &'c params::Projectile,
    scope: &'c params::Scope,
    conditions: &'c params::Conditions,
    muzzle_pitch: Numeric,
    zero_distance: Length,
    time_step: Time,
}
impl<'c> Simulation<'c> {
    pub(crate) fn new(
        projectile: &'c params::Projectile,
        scope: &'c params::Scope,
        conditions: &'c params::Conditions,
        muzzle_pitch: Numeric,
        zero_distance: Length,
        time_step: Numeric,
    ) -> Self {
        Self {
            projectile,
            scope,
            conditions,
            muzzle_pitch,
            zero_distance,
            time_step: Time::Seconds(time_step),
        }
    }
    // Rotated velocity vector, accounts for muzzle/shooter pitch, and yaw (bearing)
    // Start with velocity value along X unit vector
    fn muzzle_velocity_vector(&self) -> Vector3<Numeric> {
        self.projectile
            .velocity()
            .pitch(self.conditions.other.line_of_sight() + self.muzzle_pitch)
            .yaw(self.conditions.other.azimuth())
    }
    // Velocity vector of wind, right now calculated only for horizontal winds.  Can add another
    // factor, wind_pitch, to consider vertical wind components
    fn wind_velocity_vector(&self) -> Vector3<Numeric> {
        self.conditions
            .wind
            .velocity()
            .yaw(self.conditions.wind.yaw() + self.conditions.other.azimuth())
    }
}
