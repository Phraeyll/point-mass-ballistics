pub use self::{
    error::{Error, ErrorKind, Result},
    iter::Iter,
    output::Measurements,
    simulation::{Bc, BcKind, Simulation, SimulationBuilder},
    util::*,
};

#[macro_use]
#[allow(clippy::or_fun_call)]
#[allow(clippy::let_and_return)]
mod util;

#[macro_use]
mod vectors;

mod error;
#[allow(clippy::float_cmp)]
mod iter;
mod output;
mod physics;
mod simulation;
pub mod solvers {
    pub use self::zero::*;
    #[allow(clippy::float_cmp)]
    #[allow(clippy::nonminimal_bool)]
    pub mod zero;
}
#[allow(clippy::approx_constant)]
mod dragtables {
    pub mod g1;
    pub mod g2;
    pub mod g5;
    pub mod g6;
    pub mod g7;
    pub mod g8;
    pub mod gi;
    pub mod gs;
}
