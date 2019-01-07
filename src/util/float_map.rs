use ordered_float::OrderedFloat;

use super::Numeric;

use std::collections::BTreeMap;

pub struct FloatMap<T>(pub BTreeMap<OrderedFloat<Numeric>, T>);
impl<T> Default for FloatMap<T> {
    fn default() -> Self {
        FloatMap(BTreeMap::new())
    }
}
impl<T> FloatMap<T> {
    pub fn new() -> Self {
        FloatMap::default()
    }
}

// Create wrapped btreemap representation of drag tables from vector representation
// May consider parsing from a file, but I think it would be better to bundle tables inside
// the binary, rather than reducing performance due to IO access
// Consider adding another enum variant for custom table construction
impl FloatMap<Numeric> {
    // Linear interpolation of point 'mach' and associated CD
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
