use self::WeightMass::*;

pub const LBS_TO_GRAINS: f64 = 7000.0;
pub const GRAINS_TO_LBS: f64 = 1.0 / LBS_TO_GRAINS;

pub const GRAINS_TO_KGS: f64 = GRAINS_TO_LBS * LBS_TO_KGS;
pub const KGS_TO_GRAINS: f64 = 1.0 / GRAINS_TO_KGS;

pub const LBS_TO_KGS: f64 = 0.45359237;
pub const KGS_TO_LBS: f64 = 1.0 / LBS_TO_KGS;

#[derive(Debug, Copy, Clone)]
pub enum WeightMass {
    Grains(f64),
    Lbs(f64),
    Kgs(f64),
}
impl From<WeightMass> for f64 {
    fn from(u: WeightMass) -> f64 {
        match u {
            Grains(u) => u,
            Lbs(u) => u,
            Kgs(u) => u,
        }
    }
}
impl self::WeightMass {
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
