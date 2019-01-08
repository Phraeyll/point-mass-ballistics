use ordered_float::OrderedFloat;

use super::Numeric;

use std::{collections::{btree_map, BTreeMap}, iter::FromIterator};

pub struct FloatMap<V>(pub BTreeMap<OrderedFloat<Numeric>, V>);

impl<V> Default for FloatMap<V> {
    fn default() -> Self {
        FloatMap::new()
    }
}

impl<V> FromIterator<(Numeric, V)> for FloatMap<V> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Numeric, V)>,
    {
        FloatMap(BTreeMap::from_iter(
            iter.into_iter().map(|(key, val)| (OrderedFloat(key), val)),
        ))
    }
}

impl<V> FloatMap<V> {
    pub fn new() -> Self {
        FloatMap(BTreeMap::new())
    }
    pub fn insert(&mut self, key: Numeric, value: V) -> Option<V> {
        self.0.insert(OrderedFloat(key), value)
    }
    pub fn iter(&self) -> btree_map::Iter<OrderedFloat<Numeric>, V> {
        self.0.iter()
    }
}


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
        let mut _float_map = FloatMap::new();
        $(
            let _ = _float_map.insert($key, $val);
        )*
        _float_map
    }};
}
