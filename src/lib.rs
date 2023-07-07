pub type Numeric = f64;
const OPTIMIZE_DRAG_TABLE: bool = true;

mod consts;
mod physics;
mod vectors;

#[allow(clippy::approx_constant)]
pub mod drag;
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
