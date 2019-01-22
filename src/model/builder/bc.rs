use super::dragtables::*;
use crate::model::builder::BcBuilder;
use crate::model::core::{Bc, BcKind, BcKind::*};
use crate::util::*;

impl Default for Bc {
    fn default() -> Self {
        // Arbitrary data - intended to be set by with method above at initialization point
        Self {
            value: 0.0,
            kind: G1,
            table: float_map![],
        }
    }
}

impl BcBuilder for Bc {
    fn with(value: Numeric, kind: BcKind) -> Result<Self> {
        if value.is_sign_positive() {
            Ok(Self {
                value,
                kind,
                table: match kind {
                    G1 => g1::init(),
                    G2 => g2::init(),
                    G5 => g5::init(),
                    G6 => g6::init(),
                    G7 => g7::init(),
                    G8 => g8::init(),
                    GI => gi::init(),
                    GS => gs::init(),
                },
            })
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
}
