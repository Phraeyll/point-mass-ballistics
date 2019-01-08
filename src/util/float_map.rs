use ordered_float::OrderedFloat;

use super::Numeric;

use std::collections::BTreeMap;

pub struct FloatMap<T>(pub BTreeMap<OrderedFloat<Numeric>, T>);

impl FloatMap<Numeric> {
    // Linear interpolation for 'y' of value 'x'
    // Search for closest surrounding 'x' keys in map
    // and use them along with their values for interpolation
    // Works for exact values of 'x' as well
    pub fn lerp(&self, x: Numeric) -> Numeric {
        self.0
            .range(OrderedFloat(x)..)
            .zip(self.0.range(..OrderedFloat(x)).rev())
            .next()
            .map(|((OrderedFloat(x1), y1), (OrderedFloat(x0), y0))| {
                y0 + (x - x0) * ((y1 - y0) / (x1 - x0))
            })
            .expect("Velocity out of range")
    }
}

// Initialize BTreeMap with OrdereredFloat wrapper around key, and FloatMap wrapper
// around entire map.  Used for drag tables and output/drop tables
macro_rules! float_map {
    ( $($key:expr => $val:expr,)+ ) => {
        float_map![
            $($key => $val),+
        ]
    };
    ( $($key:expr => $val:expr),* ) => {{
        let mut _float_map = FloatMap(BTreeMap::new());
        $(
            let _ = _float_map.0.insert(OrderedFloat($key), $val);
        )*
        _float_map
    }};
}
