use crate::{
    error::{Error, Result},
    simulation::SectionalDensity,
    util::{pound, square_inch, Area, Mass, Numeric, NumericMap},
};

use lazy_static::lazy_static;

mod g1;
mod g2;
mod g5;
mod g6;
mod g7;
mod g8;
mod gi;
mod gs;

pub trait DragTable {
    fn new(value: Numeric) -> Self;
    fn value(&self) -> SectionalDensity;
    fn cd(&self, x: Numeric) -> Result<Numeric>;
}

macro_rules! drag_tables {
    ($($struct:ident => $expr:expr,)+) => {
        drag_tables!{$($struct => $expr),+}
    };
    ($($struct:ident => $expr:expr),*) => {
        $(
            pub struct $struct {
                value: SectionalDensity,
            }
            impl DragTable for $struct {
                fn new(value: Numeric) -> Self {
                    Self {
                        value: Mass::new::<pound>(value) / Area::new::<square_inch>(1.0),
                    }
                }
                fn value(&self) -> SectionalDensity {
                    self.value
                }
                // Linear interpolation for 'y' of value 'x'
                // Search for closest surrounding 'x' ks in map
                // and use them along with their values for interpolation
                // Works for exact values of 'x' as well
                fn cd(&self, x: Numeric) -> Result<Numeric> {
                    lazy_static! {
                        static ref TABL: NumericMap = $expr;
                    }
                    TABL.range(..x)
                        .rev()
                        .zip(TABL.range(x..))
                        .map(|((x0, &y0), (x1, &y1))| y0 + (x - x0) * ((y1 - y0) / (x1 - x0)))
                        .next()
                        .ok_or(Error::VelocityLookup(x))
                }
            }
        )*
    };
}

drag_tables! {
    G1 => g1::table(),
    G2 => g2::table(),
    G5 => g5::table(),
    G6 => g6::table(),
    G7 => g7::table(),
    G8 => g8::table(),
    GI => gi::table(),
    GS => gs::table(),
}
