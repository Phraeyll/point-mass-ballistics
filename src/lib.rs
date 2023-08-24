pub type Numeric = f64;

mod consts {
    pub use std::f64::consts::{
        PI,
        FRAC_PI_2,
        FRAC_PI_4,
        FRAC_PI_8,
    };
}
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
