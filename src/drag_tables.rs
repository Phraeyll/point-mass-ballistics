use crate::{error::Result, simulation::SectionalDensity, util::Numeric};

pub mod g1;
pub mod g2;
pub mod g5;
pub mod g6;
pub mod g7;
pub mod g8;
pub mod gi;
pub mod gs;

pub trait DragTable {
    fn new(value: Numeric) -> Self;
    fn value(&self) -> SectionalDensity;
    fn cd(&self, x: Numeric) -> Result<Numeric>;
}
