use crate::{
    error::Result,
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
                fn cd(&self, x: Numeric) -> Result<Numeric> {
                    lazy_static! {
                        static ref TABLE: NumericMap = $expr;
                    }
                    TABLE.lerp(x)
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
