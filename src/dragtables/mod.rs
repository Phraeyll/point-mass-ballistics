use of::OrderedFloat;

pub use self::BallisticCoefficient::*;

use std::collections::BTreeMap;

mod g1;
mod g2;
mod g5;
mod g6;
mod g7;
mod g8;
mod gi;

// Wrapper around drag tables map
#[derive(Debug)]
pub struct DragTable(pub BTreeMap<OrderedFloat<f64>, f64>);

pub enum BallisticCoefficient {
    G1(f64),
    G2(f64),
    G5(f64),
    G6(f64),
    G7(f64),
    G8(f64),
    GI(f64),
}

// Unwrap BC and create associated drag table
impl BallisticCoefficient {
    pub fn create(self) -> (f64, DragTable) {
        match self {
            G1(bc) => (bc, g1::init()),
            G2(bc) => (bc, g2::init()),
            G5(bc) => (bc, g5::init()),
            G6(bc) => (bc, g6::init()),
            G7(bc) => (bc, g7::init()),
            G8(bc) => (bc, g8::init()),
            GI(bc) => (bc, gi::init()),
        }
    }
}

// Create wrapped btreemap representation of drag tables from vector representation
// May consider parsing from a file, but I think it would be better to bundle tables inside
// the binary, rather than reducing performance due to IO access
// Consider adding another enum variant for custom table construction
impl DragTable {
    // Linear interpolation of point 'mach' and associated CD
    pub fn lerp(&self, x: f64) -> f64 {
        let (OrderedFloat(x0), y0) = self.0.range(..OrderedFloat(x)).next_back().unwrap();
        let (OrderedFloat(x1), y1) = self.0.range(OrderedFloat(x)..).next().unwrap();
        y0 + (x - x0) * (y1 - y0) / (x1 - x0)
    }
}
