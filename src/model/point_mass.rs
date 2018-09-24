pub use self::dragtables::BallisticCoefficient;

use crate::util::Numeric;

pub mod params;
pub(crate) mod iter;
pub(crate) mod zero;
mod dragtables;

pub struct Simulation<'mc> {
    params: &'mc params::Unconditional,
    conditions: &'mc params::Conditional,
    muzzle_pitch: Numeric,
}
impl<'mc> Simulation<'mc> {
    pub fn new(
        params: &'mc params::Unconditional,
        conditions: &'mc params::Conditional,
        muzzle_pitch: Numeric,
    ) -> Self {
        Self {
            params,
            conditions,
            muzzle_pitch,
        }
    }
}
