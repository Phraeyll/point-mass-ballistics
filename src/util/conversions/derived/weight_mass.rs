use self::WeightMass::*;
use crate::util::Numeric;

pub(super) const LBS_TO_GRAINS: Numeric = 7_000.0;
pub(super) const GRAINS_TO_LBS: Numeric = 1.0 / LBS_TO_GRAINS;

pub(super) const LBS_TO_KGS: Numeric = 0.453_592_37;
pub(super) const KGS_TO_LBS: Numeric = 1.0 / LBS_TO_KGS;

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
impl WeightMass {
    pub fn to_num(self) -> Numeric {
        From::from(self)
    }
    pub fn to_grains(self) -> Self {
        match self {
            u @ Grains(_) => u,
            u @ Kgs(_) => u.to_lbs().to_grains(),
            Lbs(u) => Grains(u * LBS_TO_GRAINS),
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
            u @ Grains(_) => u.to_lbs().to_kgs(),
            Lbs(u) => Kgs(u * LBS_TO_KGS),
        }
    }
}
