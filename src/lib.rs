pub use self::{
    drag_tables::DragTable,
    error::{Error, Result},
    iter::Iter,
    output::*,
    simulation::{Simulation, SimulationBuilder},
    util::*,
};

#[macro_use]
mod util;
#[macro_use]
mod vectors;
mod error;
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
