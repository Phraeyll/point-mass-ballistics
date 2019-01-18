use WeightMass::*;
use crate::util::Numeric;

pub const LBS_TO_GRAINS: Numeric = 7_000.0;
pub const GRAINS_TO_LBS: Numeric = 1.0 / LBS_TO_GRAINS;

pub const GRAINS_TO_KGS: Numeric = GRAINS_TO_LBS * LBS_TO_KGS;
pub const KGS_TO_GRAINS: Numeric = 1.0 / GRAINS_TO_KGS;

pub const LBS_TO_KGS: Numeric = 0.453_592_37;
pub const KGS_TO_LBS: Numeric = 1.0 / LBS_TO_KGS;

#[derive(Debug, Copy, Clone)]
pub enum WeightMass {
    Grains(Numeric),
    Lbs(Numeric),
    Kgs(Numeric),
}
impl From<WeightMass> for Numeric {
    fn from(u: WeightMass) -> Numeric {
        match u {
            Grains(u) => u,
            Lbs(u) => u,
            Kgs(u) => u,
        }
    }
}
impl self::WeightMass {
    pub fn to_num(self) -> Numeric {
        Numeric::from(self)
    }
    pub fn to_grains(self) -> Self {
        match self {
            u @ Grains(_) => u,
            Lbs(u) => Grains(u * LBS_TO_GRAINS),
            Kgs(u) => Grains(u * KGS_TO_GRAINS),
        }
    }
    pub fn to_lbs(self) -> Self {
        match self {
            u @ Lbs(_) => u,
            Grains(u) => Lbs(u * GRAINS_TO_LBS),
            Kgs(u) => Lbs(u * KGS_TO_LBS),
        }
    }
    pub fn to_kgs(self) -> Self {
        match self {
            u @ Kgs(_) => u,
            Grains(u) => Kgs(u * GRAINS_TO_KGS),
            Lbs(u) => Kgs(u * LBS_TO_KGS),
        }
    }
}
