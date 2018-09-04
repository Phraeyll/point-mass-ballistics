use of::OrderedFloat;

pub use self::DragTableKind::*;

use std::collections::BTreeMap;

custom_derive! {
    #[derive(EnumFromStr)]
    pub enum DragTableKind {
        G1,
        G2,
        G5,
        G6,
        G7,
        G8,
        GI,
    }
}

// Drag table functions which return vector of tuples representing drag tables
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
    pub fn new(drag_table_kind: DragTableKind) -> Self {
        let mut drag_table = DragTable(BTreeMap::new());
        let v: Vec<(f64, f64)> = match drag_table_kind {
            G1 => g1::init(),
            G2 => g2::init(),
            G5 => g5::init(),
            G6 => g6::init(),
            G7 => g7::init(),
            G8 => g8::init(),
            GI => gi::init(),
        };
        drag_table.mut_iter_insert(v);
        drag_table
    }
}
