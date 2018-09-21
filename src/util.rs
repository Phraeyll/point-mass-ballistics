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
