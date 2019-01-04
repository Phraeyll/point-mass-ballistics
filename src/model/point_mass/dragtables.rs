pub use self::BallisticCoefficient::*;

use crate::util::{FloatMap, Numeric};

mod g1;
mod g2;
mod g5;
mod g6;
mod g7;
mod g8;
mod gi;
mod gs;

// Type of BC used, implies which drag table to use
#[derive(Copy, Clone)]
pub enum BallisticCoefficient {
    G1(Numeric),
    G2(Numeric),
    G5(Numeric),
    G6(Numeric),
    G7(Numeric),
    G8(Numeric),
    GI(Numeric),
    GS(Numeric),
}

// Unwrap BC and create associated drag table
impl BallisticCoefficient {
    pub(crate) fn to_num(self) -> Numeric {
        Numeric::from(self)
    }
    pub(crate) fn table(self) -> FloatMap<Numeric> {
        match self {
            G1(_) => g1::init(),
            G2(_) => g2::init(),
            G5(_) => g5::init(),
            G6(_) => g6::init(),
            G7(_) => g7::init(),
            G8(_) => g8::init(),
            GI(_) => gi::init(),
            GS(_) => gs::init(),
        }
    }
}

impl From<BallisticCoefficient> for Numeric {
    fn from(u: BallisticCoefficient) -> Numeric {
        match u {
            G1(u) => u,
            G2(u) => u,
            G5(u) => u,
            G6(u) => u,
            G7(u) => u,
            G8(u) => u,
            GI(u) => u,
            GS(u) => u,
        }
    }
}
