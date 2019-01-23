use crate::model::core::{Bc, BcKind::*};
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
