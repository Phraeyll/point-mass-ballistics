pub type Numeric = f64;
pub type NumericMap = float_map::FloatMap<Numeric>;

#[macro_use]
mod float_map;
mod consts;
mod physics;
mod vectors;

pub mod error;
pub mod iter;
pub mod output;
#[allow(clippy::approx_constant)]
pub mod projectiles;
pub mod simulation;
pub mod units;
pub mod solvers {
    pub use self::zero::*;
    #[allow(clippy::float_cmp)]
    #[allow(clippy::nonminimal_bool)]
    pub mod zero;
}
