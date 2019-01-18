pub use BallisticCoefficientVariant::*;

use crate::util::*;

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
pub enum BallisticCoefficientVariant {
    G1,
    G2,
    G5,
    G6,
    G7,
    G8,
    GI,
    GS,
}

pub struct BallisticCoefficient {
    value: Numeric,
    variant: BallisticCoefficientVariant,
    table: FloatMap<Numeric>,
}

// Unwrap BC and create associated drag table
impl BallisticCoefficient {
    pub fn new(value: Numeric, variant: BallisticCoefficientVariant) -> Self {
        Self {
            value,
            variant,
            table: match variant {
                G1 => g1::init(),
                G2 => g2::init(),
                G5 => g5::init(),
                G6 => g6::init(),
                G7 => g7::init(),
                G8 => g8::init(),
                GI => gi::init(),
                GS => gs::init(),
            },
        }
    }
    pub fn value(&self) -> Numeric {
        self.value
    }
    pub fn table(&self) -> &FloatMap<Numeric> {
        &self.table
    }
    pub fn variant(&self) -> BallisticCoefficientVariant {
        self.variant
    }
}
