// Determine which type to use dynamically, accounts for f32/f64 consts as well.
macro_rules! my_type {
    ( $t:ident ) => {
        use std::$t::consts;
        pub type Numeric = $t;
    };
}
my_type!(f64);
pub const PI: Numeric = consts::PI;
pub const FRAC_PI_4: Numeric = consts::FRAC_PI_4;
pub const FRAC_PI_2: Numeric = consts::FRAC_PI_2;

use ordered_float::OrderedFloat;

use std::collections::BTreeMap;

pub struct FloatMap<T>(pub BTreeMap<OrderedFloat<Numeric>, T>);
impl<T> FloatMap<T> {
    pub fn default() -> Self {
        FloatMap(BTreeMap::new())
    }
}

// Create wrapped btreemap representation of drag tables from vector representation
// May consider parsing from a file, but I think it would be better to bundle tables inside
// the binary, rather than reducing performance due to IO access
// Consider adding another enum variant for custom table construction
impl FloatMap<Numeric> {
    // Linear interpolation of point 'mach' and associated CD
    pub fn lerp(&self, x: Numeric) -> Numeric {
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

macro_rules! float_map {
    ( $($mach:expr => $cd:expr,)+ ) => ( float_map!($($mach => $cd),+) );
    ( $($mach:expr => $cd:expr),* ) => {{
        let mut _float_map = FloatMap(BTreeMap::new());
        $(
            let _ = _float_map.0.insert(OrderedFloat($mach), $cd);
        )*
        _float_map
    }}
}
// Rationals, and generic numeric types are going to require much more work
// use num::{Rational, FromPrimitive};
// pub type Numeric = Rational;
// use std::f64::consts;
// pub const PI: Numeric = Rational::from_f64(consts::PI).unwrap();
// pub const FRAC_PI_4: Numeric = Rational::from_f64(consts::FRAC_PI_4).unwrap();

// Modified from Rust core TakeWhile
// Also takes the first item which breaks predicate

pub trait MyIterators {
    type Item;
    fn take_do_while<P>(self, predicate: P) -> TakeDoWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        TakeDoWhile {
            iter: self,
            flag: false,
            predicate,
        }
    }
}

impl<I: Iterator> MyIterators for I {
    type Item = I::Item;
}

pub struct TakeDoWhile<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I: Iterator, P> Iterator for TakeDoWhile<I, P>
where
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.flag {
            None
        } else {
            self.iter.next().and_then(|x| {
                if (self.predicate)(&x) {
                    Some(x)
                } else {
                    self.flag = true;
                    Some(x)
                }
            })
        }
    }
}
