#[macro_use]
mod util {
    // Determine which type to use dynamically, accounts for f32/f64 consts as well.
    // Can't use ident - need to check token to determine Natural type
    macro_rules! numeric {
        ( f32 ) => {
            use std::f32::consts;
            pub type Numeric = f32;
            pub type Natural = u16;
        };
        ( f64 ) => {
            use std::f64::consts;
            pub type Numeric = f64;
            pub type Natural = u32;
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
