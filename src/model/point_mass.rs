pub use self::dragtables::BallisticCoefficient;

use crate::util::{conversions::*, Numeric};

pub(crate) mod iter;
pub mod params;

#[allow(clippy::approx_constant)]
mod dragtables;

#[allow(clippy::float_cmp)]
pub(crate) mod zero;

pub(crate) struct Simulation<'c> {
    projectile: &'c params::Projectile,
    scope: &'c params::Scope,
    wind: &'c params::Wind,
    atmosphere: &'c params::Atmosphere,
    conditions: &'c params::Conditions,
    muzzle_pitch: Numeric,
    time_step: Time,
}
impl<'c> Simulation<'c> {
    pub(crate) fn new(
        projectile: &'c params::Projectile,
        scope: &'c params::Scope,
        wind: &'c params::Wind,
        atmosphere: &'c params::Atmosphere,
        conditions: &'c params::Conditions,
        muzzle_pitch: Numeric,
        time_step: Numeric,
    ) -> Self {
        Self {
            projectile,
            scope,
            wind,
            atmosphere,
            conditions,
            muzzle_pitch,
            time_step: Time::Seconds(time_step),
        }
    }
    pub(crate) fn muzzle_pitch(&self) -> Numeric {
        self.muzzle_pitch.to_radians()
    }
}
