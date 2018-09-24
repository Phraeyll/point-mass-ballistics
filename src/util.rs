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

pub use self::conversions::*;
pub use self::float_map::*;
pub use self::iter_adaptors::*;
pub use self::nalgebra_helpers::*;

#[macro_use]
pub mod float_map;
pub mod conversions;
pub mod iter_adaptors;
pub mod nalgebra_helpers;
