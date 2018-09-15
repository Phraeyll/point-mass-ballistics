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
mod gs;

// Wrapper around drag tables map
#[derive(Debug)]
pub struct DragTable(pub BTreeMap<OrderedFloat<f64>, f64>);

// Type of BC used, implies which drag table to use
#[derive(Copy, Clone)]
pub enum BallisticCoefficient {
    G1(f64),
    G2(f64),
    G5(f64),
    G6(f64),
    G7(f64),
    G8(f64),
    GI(f64),
    GS(f64),
}

// Unwrap BC and create associated drag table
impl BallisticCoefficient {
    pub fn table(self) -> DragTable {
        match self {
            G1(_) => g1::init(),
            G2(_) => g2::init(),
            G5(_) => g5::init(),
            G6(_) => g6::init(),
            G7(_) => g7::init(),
            G8(_) => g8::init(),
            GI(_) => gi::init(),
            GS(_) => gs::init(),
        }
    }
}

impl From<BallisticCoefficient> for f64 {
    fn from(u: BallisticCoefficient) -> f64 {
        match u {
            G1(u) => u,
            G2(u) => u,
            G5(u) => u,
            G6(u) => u,
            G7(u) => u,
            G8(u) => u,
            GI(u) => u,
            GS(u) => u,
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
        let (x0, y0) = match self.0.range(..OrderedFloat(x)).next_back() {
            Some((OrderedFloat(key), val)) => (key, val),
            None => panic!("Velocity out of range!"),
        };
        let (x1, y1) = match self.0.range(OrderedFloat(x)..).next() {
            Some((OrderedFloat(key), val)) => (key, val),
            None => panic!("Velocity out of range!"),
        };
        y0 + (x - x0) * ((y1 - y0) / (x1 - x0))
    }
}
