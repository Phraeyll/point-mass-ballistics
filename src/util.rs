use crate::error::{Error, Result};

use std::{
    collections::BTreeMap,
    f64::consts,
    iter::FromIterator,
    ops::{Bound, RangeBounds},
};

use ordered_float::OrderedFloat;
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

pub struct IntoIter<V>(<BTreeMap<OrderedFloat<Numeric>, V> as IntoIterator>::IntoIter);

impl<V> Iterator for IntoIter<V> {
    type Item = (Numeric, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(OrderedFloat(key), val)| (key, val))
    }
}

impl<V> IntoIterator for FloatMap<V> {
    type IntoIter = IntoIter<V>;
    type Item = <Self::IntoIter as Iterator>::Item;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

impl<V> FromIterator<<Self as IntoIterator>::Item> for FloatMap<V> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = <Self as IntoIterator>::Item>,
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
    pub fn iter(&self) -> impl Iterator<Item = (&Numeric, &V)> {
        self.0.iter().map(|(OrderedFloat(key), val)| (key, val))
    }
    pub fn range<R>(&self, range: R) -> impl DoubleEndedIterator<Item = (&Numeric, &V)>
    where
        R: RangeBounds<Numeric>,
    {
        use Bound::*;
        fn wrap_bound(bound: Bound<&Numeric>) -> Bound<OrderedFloat<Numeric>> {
            match bound {
                Unbounded => Unbounded,
                Excluded(f) => Excluded(OrderedFloat(*f)),
                Included(f) => Included(OrderedFloat(*f)),
            }
        }
        let start = wrap_bound(range.start_bound());
        let end = wrap_bound(range.end_bound());

        self.0
            .range((start, end))
            .map(|(OrderedFloat(key), val)| (key, val))
    }
}

impl FloatMap<Numeric> {
    // Linear interpolation for 'y' of value 'x'
    // Search for closest surrounding 'x' keys in map
    // and use them along with their values for interpolation
    // Works for exact values of 'x' as well
    pub fn lerp(&self, x: Numeric) -> Result<Numeric> {
        self.range(..x)
            .rev()
            .zip(self.range(x..))
            .next()
            .map(|((x0, y0), (x1, y1))| y0 + (x - x0) * ((y1 - y0) / (x1 - x0)))
            .ok_or(Error::VelocityLookup(x))
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
        let mut _float_map = $crate::FloatMap::new();
        $(
            let _ = _float_map.insert($key, $val);
        )*
        _float_map
    }};
}
