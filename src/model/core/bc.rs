use crate::model::core::dragtables::*;
use crate::util::*;
pub use BcKind::*;

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
    Null,
}
#[derive(Debug)]
pub struct Bc {
    pub(crate) value: Numeric,
    pub(crate) kind: BcKind,
    pub(crate) table: FloatMap<Numeric>,
}
#[derive(Debug)]
pub struct BcBuilder {
    pub value: Numeric,
    pub kind: BcKind,
    pub table: FloatMap<Numeric>,
}
impl From<BcBuilder> for Bc {
    fn from(other: BcBuilder) -> Self {
        Self {
            value: other.value,
            kind: other.kind,
            table: other.table,
        }
    }
}
impl From<Bc> for BcBuilder {
    fn from(other: Bc) -> Self {
        Self {
            value: other.value,
            kind: other.kind,
            table: other.table,
        }
    }
}
impl Default for BcBuilder {
    fn default() -> Self {
        // Arbitrary data - intended to be set by with method above at initialization point
        Self {
            value: 0.0,
            kind: Null,
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
impl BcBuilder {
    pub fn new(value: Numeric, kind: BcKind) -> BcBuilder {
        BcBuilder {
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
                Null => float_map![],
            },
        }
    }
}
