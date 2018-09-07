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
// May consider parsing from a file, but I think it would be better to bundle inside the binary
// Rather than reducing performance due to IO access
impl DragTable {
    fn mut_iter_insert(&mut self, mach_cd_values: Vec<(f64, f64)>) {
        for (x, y) in mach_cd_values.into_iter() {
            self.0.insert(OrderedFloat(x), y);
        }
    }
    // Linear interpolation of point 'mach' and associated CD
    pub fn lerp(&self, mach: f64) -> f64 {
        let key = OrderedFloat(mach);
        let (x0, y0) = self.0.range(..key).next_back().unwrap();
        let (x1, y1) = self.0.range(key..).next().unwrap();
        let (x, y) = ((x0.0, x1.0), (*y0, *y1));
        y.0 + (mach - x.0) * (y.1 - y.0) / (x.1 - x.0)
    }
    // Initialize table from enum passed in from caller
    pub fn new(v: Vec<(f64, f64)>) -> Self {
        let mut drag_table = DragTable(BTreeMap::new());
        drag_table.mut_iter_insert(v);
        drag_table
    }
}
