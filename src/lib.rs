#[macro_use]
mod util {
    pub use self::{conversions::*, float_map::*, nalgebra_helpers::*};
    pub use crate::error::*;

    use std::f64::consts;

    pub type Numeric = f64;
    pub type Natural = u32;

    pub const PI: Numeric = consts::PI;
    pub const FRAC_PI_4: Numeric = consts::FRAC_PI_4;
    pub const FRAC_PI_2: Numeric = consts::FRAC_PI_2;

    #[macro_use]
    #[allow(clippy::or_fun_call)]
    #[allow(clippy::let_and_return)]
    mod float_map;
    pub mod conversions {
        // Terribly inefficient and unsafe/untyped method of unit conversion, only for units needed
        // Really need to replace with some form of dimensional analysis.  May be able to use crate 'uom'
        // for most conversions, but still need something for termperature.  Also, may need something
        // different for arbitrary units, such as those use in air density calculation.  uom has only
        // a few common units specified.  May be able to work around at run time.
        pub use self::{angle::*, derived::*, length::*, temperature::*, time::*, weight_mass::*};

        mod angle;
        mod derived;
        mod length;
        mod temperature;
        mod time;
        mod weight_mass;
    }
    mod nalgebra_helpers;
}
pub mod error;
pub mod model {
    pub use self::{core::*, iter::*, solver::*};

    #[allow(clippy::float_cmp)]
    pub mod iter {
        pub use self::{packet::*, simulation::*};

        mod packet;
        mod physics;
        mod simulation;
    }
    pub mod core {
        pub use self::{
            atmosphere::*, bc::*, builder::*, flags::*, projectile::*, scope::*, shooter::*,
            simulation::*, wind::*,
        };
        pub use crate::util::{conversions::*, Natural, Numeric};

        #[allow(clippy::approx_constant)]
        pub(crate) mod dragtables {
            pub mod g1;
            pub mod g2;
            pub mod g5;
            pub mod g6;
            pub mod g7;
            pub mod g8;
            pub mod gi;
            pub mod gs;
        }
        mod atmosphere;
        mod bc;
        mod builder;
        mod flags;
        mod projectile;
        mod scope;
        mod shooter;
        mod simulation;
        mod wind;
    }
    pub mod solver {
        pub use self::zero::*;

        #[allow(clippy::float_cmp)]
        #[allow(clippy::nonminimal_bool)]
        pub mod zero;
    }
}
