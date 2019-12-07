pub use self::{
    drag_tables::DragTable,
    error::{Error, Result},
    iter::Iter,
    output::*,
    simulation::{Simulation, SimulationBuilder},
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
pub mod drag_tables;
