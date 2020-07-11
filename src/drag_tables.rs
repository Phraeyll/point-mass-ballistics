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
                // TABLE is a map of "mach speed" to "coefficients of drag", {x => y}
                // This funtions returns linear approximation of coefficient, for a given mach speed
                // When x is present in the map, interpolation is equivalent to TABLE.get_value(x)
                fn cd(&self, x: Numeric) -> Result<Numeric> {
                    lazy_static! {
                        static ref TABLE: NumericMap = $expr;
                    }
                    // TODO: Does not work if x exists in map as smallest key, ..x excludes it, so first step is None
                    TABLE.range(..x).rev()     // First = None if smallest key >= x, else Some((x0, &y0)) where x0 greatest key <  x
                        .zip(TABLE.range(x..)) // First = None if greatest key <  x, else Some((x1, &y1)) where x1 smallest key >= x
                        .map(|((x0, &y0), (x1, &y1))| y0 + (x - x0) * ((y1 - y0) / (x1 - x0))) // Linear interpolation when x0 and x1 both exist
                        .next()
                        .ok_or(Error::VelocityLookup(x)) // None => Err: x is outside of key range: this function does not extrapolate
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
