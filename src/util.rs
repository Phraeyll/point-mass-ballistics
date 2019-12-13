use std::{
    collections::{btree_map, BTreeMap},
    f64::consts,
    iter::{FromIterator, FusedIterator},
    ops::{Bound, RangeBounds},
};

use ordered_float::OrderedFloat as OrdF;
pub use uom::{
    fmt::DisplayStyle,
    marker,
    si::{
        acceleration::{self, foot_per_second_squared, meter_per_second_squared},
        amount_of_substance::{self, mole},
        angle::{self, degree, minute as moa, radian},
        angular_velocity::{self, radian_per_second},
        area::{self, square_inch, square_meter},
        electric_current::{self, ampere},
        energy::{self, foot_pound, joule},
        f64::*,
        fmt::{Arguments, QuantityArguments},
        force::{self},
        length::{self, inch, meter, yard},
        luminous_intensity::{self, candela},
        mass::{self, grain, kilogram, pound},
        mass_density::{self, kilogram_per_cubic_meter},
        molar_mass::{self},
        pressure::{self, inch_of_mercury, pascal},
        ratio::{self},
        thermodynamic_temperature::{
            self as temperature, degree_celsius as celsius, degree_fahrenheit as fahrenheit, kelvin,
        },
        time::{self, second},
        velocity::{self, foot_per_second, meter_per_second, mile_per_hour},
        Dimension, Quantity, Unit, Units, ISQ, SI,
    },
    str::ParseQuantityError,
    typenum, Conversion,
};

pub type Numeric = f64;
pub type Natural = u32;
pub type NumericMap = FloatMap<Numeric>;
pub type MyUnits = SI<Numeric>;
pub type MyQuantity<D> = Quantity<D, MyUnits, Numeric>;
pub type MyQuantityArguments<D, N> = QuantityArguments<D, MyUnits, Numeric, N>;
pub const PI: Numeric = consts::PI;
pub const FRAC_PI_4: Numeric = consts::FRAC_PI_4;
pub const FRAC_PI_2: Numeric = consts::FRAC_PI_2;

// Entry
#[derive(Debug)]
pub struct Entry<'a, V>(btree_map::Entry<'a, OrdF<Numeric>, V>);

// Range
#[derive(Debug, Clone)]
pub struct Range<'a, V>(btree_map::Range<'a, OrdF<Numeric>, V>);
impl<'a, V> Iterator for Range<'a, V> {
    type Item = (Numeric, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(unwrap_ref)
    }
}
impl<'a, V> DoubleEndedIterator for Range<'a, V> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        self.0.next_back().map(unwrap_ref)
    }
}
impl<'a, V> FusedIterator for Range<'a, V> {}

// RangeMut
#[derive(Debug)]
pub struct RangeMut<'a, V>(btree_map::RangeMut<'a, OrdF<Numeric>, V>);
impl<'a, V> Iterator for RangeMut<'a, V> {
    type Item = (Numeric, &'a mut V);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(unwrap_mut)
    }
}
impl<'a, V> DoubleEndedIterator for RangeMut<'a, V> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        self.0.next_back().map(unwrap_mut)
    }
}
impl<'a, V> FusedIterator for RangeMut<'a, V> {}

// Values
#[derive(Debug, Clone)]
pub struct Values<'a, V>(btree_map::Values<'a, OrdF<Numeric>, V>);
impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a V;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
impl<'a, V> DoubleEndedIterator for Values<'a, V> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        self.0.next_back()
    }
}
impl<'a, V> ExactSizeIterator for Values<'a, V> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<'a, V> FusedIterator for Values<'a, V> {}

// ValuesMut
#[derive(Debug)]
pub struct ValuesMut<'a, V>(btree_map::ValuesMut<'a, OrdF<Numeric>, V>);
impl<'a, V> Iterator for ValuesMut<'a, V> {
    type Item = &'a mut V;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
impl<'a, V> DoubleEndedIterator for ValuesMut<'a, V> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        self.0.next_back()
    }
}
impl<'a, V> ExactSizeIterator for ValuesMut<'a, V> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<'a, V> FusedIterator for ValuesMut<'a, V> {}

// Keys
#[derive(Debug)]
pub struct Keys<'a, V>(btree_map::Keys<'a, OrdF<Numeric>, V>);
impl<'a, V> Iterator for Keys<'a, V> {
    type Item = Numeric;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|&OrdF(k)| k)
    }
}
impl<'a, V> DoubleEndedIterator for Keys<'a, V> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        self.0.next_back().map(|&OrdF(k)| k)
    }
}
impl<'a, V> ExactSizeIterator for Keys<'a, V> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<'a, V> FusedIterator for Keys<'a, V> {}

// IntoIter
pub struct FloatIntoIter<V>(btree_map::IntoIter<OrdF<Numeric>, V>);
impl<V> Iterator for FloatIntoIter<V> {
    type Item = (Numeric, V);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(unwrap_own)
    }
}
impl<V> DoubleEndedIterator for FloatIntoIter<V> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        self.0.next_back().map(unwrap_own)
    }
}
impl<V> ExactSizeIterator for FloatIntoIter<V> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<V> FusedIterator for FloatIntoIter<V> {}
impl<V> IntoIterator for FloatMap<V> {
    type IntoIter = FloatIntoIter<V>;
    type Item = <Self::IntoIter as Iterator>::Item;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            0: self.0.into_iter(),
        }
    }
}

// Iter
pub struct Iter<'a, V>(btree_map::Iter<'a, OrdF<Numeric>, V>);
impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (Numeric, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(unwrap_ref)
    }
}
impl<'a, V> DoubleEndedIterator for Iter<'a, V> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        self.0.next_back().map(unwrap_ref)
    }
}
impl<'a, V> ExactSizeIterator for Iter<'a, V> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<'a, V> FusedIterator for Iter<'a, V> {}
impl<'v, V> IntoIterator for &'v FloatMap<V> {
    type IntoIter = Iter<'v, V>;
    type Item = <Self::IntoIter as Iterator>::Item;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Iter Mut
pub struct IterMut<'a, V>(btree_map::IterMut<'a, OrdF<Numeric>, V>);
impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = (Numeric, &'a mut V);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(unwrap_mut)
    }
}
impl<'a, V> DoubleEndedIterator for IterMut<'a, V> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        self.0.next_back().map(unwrap_mut)
    }
}
impl<'a, V> ExactSizeIterator for IterMut<'a, V> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<'a, V> FusedIterator for IterMut<'a, V> {}
impl<'v, V> IntoIterator for &'v mut FloatMap<V> {
    type IntoIter = IterMut<'v, V>;
    type Item = <Self::IntoIter as Iterator>::Item;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

// Float Map

#[derive(Debug, Clone)]
pub struct FloatMap<V>(BTreeMap<OrdF<Numeric>, V>);

impl<V> Default for FloatMap<V> {
    fn default() -> Self {
        FloatMap::new()
    }
}

impl<V> FromIterator<<Self as IntoIterator>::Item> for FloatMap<V> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = <Self as IntoIterator>::Item>,
    {
        Self {
            0: iter.into_iter().map(wrap_own).collect::<_>(),
        }
    }
}

impl<V> FloatMap<V> {
    pub fn new() -> Self {
        Self { 0: BTreeMap::new() }
    }
    pub fn clear(&mut self) {
        self.0.clear()
    }
    pub fn get(&self, k: Numeric) -> Option<&V> {
        self.0.get(&OrdF(k))
    }
    pub fn get_key_value(&self, k: Numeric) -> Option<(Numeric, &V)> {
        self.0.get_key_value(&OrdF(k)).map(unwrap_ref)
    }
    pub fn contains_key(&self, k: Numeric) -> bool {
        self.0.contains_key(&OrdF(k))
    }
    pub fn get_mut(&mut self, k: Numeric) -> Option<&mut V> {
        self.0.get_mut(&OrdF(k))
    }
    pub fn insert(&mut self, k: Numeric, v: V) -> Option<V> {
        self.0.insert(OrdF(k), v)
    }
    pub fn remove(&mut self, k: Numeric) -> Option<V> {
        self.0.remove(&OrdF(k))
    }
    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0);
    }
    pub fn iter(&self) -> Iter<'_, V> {
        Iter { 0: self.0.iter() }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, V> {
        IterMut {
            0: self.0.iter_mut(),
        }
    }
    pub fn range<R>(&self, range: R) -> Range<'_, V>
    where
        R: RangeBounds<Numeric>,
    {
        let start = wrap_bound(range.start_bound());
        let end = wrap_bound(range.end_bound());

        Range {
            0: self.0.range((start, end)),
        }
    }
    pub fn range_mut<R>(&mut self, range: R) -> RangeMut<'_, V>
    where
        R: RangeBounds<Numeric>,
    {
        let start = wrap_bound(range.start_bound());
        let end = wrap_bound(range.end_bound());

        RangeMut {
            0: self.0.range_mut((start, end)),
        }
    }
    pub fn entry(&mut self, k: Numeric) -> Entry<V> {
        Entry {
            0: self.0.entry(OrdF(k)),
        }
    }
    pub fn split_off(&mut self, k: Numeric) -> Self {
        Self {
            0: self.0.split_off(&OrdF(k)),
        }
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn keys(&self) -> Keys<'_, V> {
        Keys { 0: self.0.keys() }
    }
    pub fn values(&self) -> Values<'_, V> {
        Values { 0: self.0.values() }
    }
    pub fn values_mut(&mut self) -> ValuesMut<'_, V> {
        ValuesMut {
            0: self.0.values_mut(),
        }
    }
}
fn wrap_bound(bound: Bound<&Numeric>) -> Bound<OrdF<Numeric>> {
    match bound {
        Bound::Unbounded => Bound::Unbounded,
        Bound::Excluded(f) => Bound::Excluded(OrdF(*f)),
        Bound::Included(f) => Bound::Included(OrdF(*f)),
    }
}
fn unwrap_mut<'k, 'v, V>(kv: (&'k OrdF<Numeric>, &'v mut V)) -> (Numeric, &'v mut V) {
    match kv {
        (&OrdF(k), v) => (k, v),
    }
}
fn unwrap_ref<'k, 'v, V>(kv: (&'k OrdF<Numeric>, &'v V)) -> (Numeric, &'v V) {
    match kv {
        (&OrdF(k), v) => (k, v),
    }
}
fn unwrap_own<V>(kv: (OrdF<Numeric>, V)) -> (Numeric, V) {
    match kv {
        (OrdF(k), v) => (k, v),
    }
}
fn wrap_own<V>(kv: (Numeric, V)) -> (OrdF<Numeric>, V) {
    match kv {
        (k, v) => (OrdF(k), v),
    }
}

// Initialize BTreeMap with OrdereredFloat wrapper around k, and FloatMap wrapper
// around entire map.  Used for drag tables and output/drop tables
macro_rules! float_map {
    ( $($k:expr => $v:expr,)+ ) => {
        float_map![
            $($k => $v),+
        ]
    };
    ( $($k:expr => $v:expr),* ) => {{
        let mut _float_map = $crate::FloatMap::new();
        $(
            let _ = _float_map.insert($k, $v);
        )*
        _float_map
    }};
}
