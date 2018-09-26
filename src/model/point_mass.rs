pub use self::dragtables::BallisticCoefficient;

use crate::util::{conversions::*, Numeric};

pub mod params;
pub(crate) mod iter;
#[allow(clippy::float_cmp)]
pub(crate) mod zero;
mod dragtables;

pub(crate) struct Simulation<'mc> {
    projectile: &'mc params::Projectile,
    scope: &'mc params::Scope,
    wind: &'mc params::Wind,
    atmosphere: &'mc params::Atmosphere,
    conditions: &'mc params::Conditions,
    muzzle_pitch: Numeric,
    time_step: Time,
}
impl<'mc> Simulation<'mc> {
    pub(crate) fn new(
        projectile: &'mc params::Projectile,
        scope: &'mc params::Scope,
        wind: &'mc params::Wind,
        atmosphere: &'mc params::Atmosphere,
        conditions: &'mc params::Conditions,
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
