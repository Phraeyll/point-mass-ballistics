pub use BcKind::*;
use crate::util::*;

#[derive(Debug, Copy, Clone)]
pub enum BcKind {
    G1,
    G2,
    G5,
    G6,
    G7,
    G8,
    GI,
    GS,
}
#[derive(Debug)]
pub struct Bc {
    pub(crate) value: Numeric,
    pub(crate) kind: BcKind,
    pub(crate) table: FloatMap<Numeric>,
}

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

impl Bc {
    pub fn value(&self) -> Numeric {
        self.value
    }
    pub fn table(&self) -> &FloatMap<Numeric> {
        &self.table
    }
    pub fn kind(&self) -> BcKind {
        self.kind
    }
}
