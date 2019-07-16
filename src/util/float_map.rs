use crate::{
    error::{Error, ErrorKind, Result},
    util::Numeric,
};

use std::{collections::BTreeMap, iter::FromIterator, ops::RangeBounds};

use ordered_float::OrderedFloat;

#[derive(Clone)]
pub struct FloatMap<V>(pub BTreeMap<OrderedFloat<Numeric>, V>);

impl<V> std::fmt::Debug for FloatMap<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FloatMap: [OrderedFloat(Numeric) => V, ...],")
    }
}

impl<V> Default for FloatMap<V> {
    fn default() -> Self {
        FloatMap::new()
    }
}

impl<V> IntoIterator for FloatMap<V> {
    type Item = (OrderedFloat<Numeric>, V);
    type IntoIter = <BTreeMap<OrderedFloat<Numeric>, V> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
    pub fn iter(&self) -> impl Iterator<Item = (&OrderedFloat<Numeric>, &V)> {
        self.0.iter()
    }
    pub fn range<R>(
        &self,
        range: R,
    ) -> impl DoubleEndedIterator<Item = (&OrderedFloat<Numeric>, &V)>
    where
        R: RangeBounds<OrderedFloat<Numeric>>,
    {
        self.0.range(range)
    }
}

impl FloatMap<Numeric> {
    // Linear interpolation for 'y' of value 'x'
    // Search for closest surrounding 'x' keys in map
    // and use them along with their values for interpolation
    // Works for exact values of 'x' as well
    pub fn lerp(&self, x: Numeric) -> Result<Numeric> {
        let key = OrderedFloat(x);
        self.range(..key)
            .rev()
            .zip(self.range(key..))
            .next()
            .map(|((OrderedFloat(x0), y0), (OrderedFloat(x1), y1))| {
                y0 + (x - x0) * ((y1 - y0) / (x1 - x0))
            })
            .ok_or(Error::new(ErrorKind::VelocityLookup(x)))
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
