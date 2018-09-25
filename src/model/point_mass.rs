pub use self::dragtables::BallisticCoefficient;

use crate::util::{conversions::*, Numeric};

pub mod params;
pub(crate) mod iter;
#[allow(clippy::float_cmp)]
pub(crate) mod zero;
mod dragtables;

pub(crate) struct Simulation<'mc> {
    params: &'mc params::Unconditional,
    conditions: &'mc params::Conditional,
    muzzle_pitch: Numeric,
    time_step: Time,
}
impl<'mc> Simulation<'mc> {
    pub(crate) fn new(
        params: &'mc params::Unconditional,
        conditions: &'mc params::Conditional,
        muzzle_pitch: Numeric,
        time_step: Numeric,
    ) -> Self {
        Self {
            params,
            conditions,
            muzzle_pitch,
            time_step: Time::Seconds(time_step),
        }
    }
}
