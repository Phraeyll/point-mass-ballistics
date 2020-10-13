pub type Numeric = f64;
pub type NumericMap = float_map::FloatMap<Numeric>;

pub(crate) mod consts;
#[macro_use]
mod vectors;
#[macro_use]
mod float_map;
#[allow(clippy::approx_constant)]
pub mod drag_tables;
mod physics;

pub mod error;
pub mod iter;
pub mod output;
pub mod simulation;
pub mod units;
pub mod solvers {
    pub use self::zero::*;
    #[allow(clippy::float_cmp)]
    #[allow(clippy::nonminimal_bool)]
    pub mod zero;
}
