#[macro_use]
mod util {
    // Determine which type to use dynamically, accounts for f32/f64 consts as well.
    macro_rules! numeric {
        ( $t:ident ) => {
            use std::$t::consts;
            pub type Numeric = $t;
        };
    }
    numeric!(f64);
    pub const PI: Numeric = consts::PI;
    pub const FRAC_PI_4: Numeric = consts::FRAC_PI_4;
    pub const FRAC_PI_2: Numeric = consts::FRAC_PI_2;

    pub use conversions::*;
    pub use float_map::*;
    pub use nalgebra_helpers::*;

    #[macro_use]
    mod float_map;
    mod conversions;
    mod nalgebra_helpers;
}

pub mod model {
    pub mod point_mass;
}
