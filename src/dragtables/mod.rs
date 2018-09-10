use of::OrderedFloat;

pub use self::BallisticCoefficient::*;

use std::collections::BTreeMap;

// Drag table functions which return vector of tuples representing drag tables
mod g1;
mod g2;
mod g5;
mod g6;
mod g7;
mod g8;
mod gi;

pub enum BallisticCoefficient {
    G1(f64),
    G2(f64),
    G5(f64),
    G6(f64),
    G7(f64),
    G8(f64),
    GI(f64),
}

impl BallisticCoefficient {
    pub fn create(self) -> (f64, DragTable) {
        match self {
            G1(bc) => (bc, DragTable::new(g1::init())),
            G2(bc) => (bc, DragTable::new(g2::init())),
            G5(bc) => (bc, DragTable::new(g5::init())),
            G6(bc) => (bc, DragTable::new(g6::init())),
            G7(bc) => (bc, DragTable::new(g7::init())),
            G8(bc) => (bc, DragTable::new(g8::init())),
            GI(bc) => (bc, DragTable::new(gi::init())),
        }
    }
}

// Wrapper around drag tables map
#[derive(Debug)]
pub struct DragTable(pub BTreeMap<OrderedFloat<f64>, f64>);

// Create wrapped btreemap representation of drag tables from vector representation
// May consider parsing from a file, but I think it would be better to bundle tables inside
// the binary, rather than reducing performance due to IO access
// Consider adding another enum variant for custom table construction
impl DragTable {
    // Initialize table from enum passed in from caller
    pub fn new(mach_cd_values: Vec<(f64, f64)>) -> Self {
        let mut drag_table = DragTable(BTreeMap::new());
        for (x, y) in mach_cd_values.into_iter() {
            drag_table.0.insert(OrderedFloat(x), y);
        }
        drag_table
    }
    // Linear interpolation of point 'mach' and associated CD
    pub fn lerp(&self, mach: f64) -> f64 {
        let key = OrderedFloat(mach);
        let (OrderedFloat(x0), y0) = self.0.range(..key).next_back().unwrap();
        let (OrderedFloat(x1), y1) = self.0.range(key..).next().unwrap();
        y0 + (mach - x0) * (y1 - y0) / (x1 - x0)
    }
}
