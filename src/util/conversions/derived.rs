use self::{
    length::*, time::*, weight_mass::*, Acceleration::*, Density::*, Energy::*, Pressure::*,
    Velocity::*,
};
use crate::util::Numeric;

pub(super) mod angle;
pub(super) mod length;
pub(super) mod temperature;
pub(super) mod time;
pub(super) mod weight_mass;

//Energy
const JOULE_TO_FTLB: Numeric = 0.737_56;
const FTLB_TO_JOULE: Numeric = 1.0 / JOULE_TO_FTLB;

// Pressure
const INHG_TO_PASCAL: Numeric = 3_386.38;
const PASCAL_TO_INHG: Numeric = 1.0 / INHG_TO_PASCAL;

// Density
const LBPF3_TO_KGPM3: Numeric = LBS_TO_KGS / (FEET_TO_METERS * FEET_TO_METERS * FEET_TO_METERS);
const KGPM3_TO_LBPF3: Numeric = 1.0 / LBPF3_TO_KGPM3;

// Velocity
const MPH_TO_MPS: Numeric = MPH_TO_FPS * FPS_TO_MPS;
const MPS_TO_MPH: Numeric = 1.0 / MPH_TO_MPS;

const MPH_TO_FPS: Numeric = (MILES_TO_YARDS * YARDS_TO_FEET) / (HOURS_TO_MINUTES * MINUTES_TO_SECONDS);
const FPS_TO_MPH: Numeric = 1.0 / MPH_TO_FPS;

const FPS_TO_MPS: Numeric = FEET_TO_METERS;
const MPS_TO_FPS: Numeric = 1.0 / FPS_TO_MPS;

// Acceleration ??
const MPH2_TO_MPS2: Numeric = MPH2_TO_FPS2 * FPS2_TO_MPS2;
const MPS2_TO_MPH2: Numeric = 1.0 / MPH2_TO_MPS2;

const MPH2_TO_FPS2: Numeric = MPH_TO_FPS / HOURS_TO_SECONDS;
const FPS2_TO_MPH2: Numeric = 1.0 / MPH2_TO_FPS2;

const FPS2_TO_MPS2: Numeric = FPS_TO_MPS;
const MPS2_TO_FPS2: Numeric = 1.0 / FPS2_TO_MPS2;

#[derive(Debug, Copy, Clone)]
pub enum Energy {
    Joules(Numeric),
    Ftlbs(Numeric),
}
impl From<Energy> for Numeric {
    fn from(u: Energy) -> Numeric {
        match u {
            Joules(u) => u,
            Ftlbs(u) => u,
        }
    }
}
impl Energy {
    pub fn to_num(self) -> Numeric {
        From::from(self)
    }
    pub fn to_joules(self) -> Self {
        match self {
            u @ Joules(_) => u,
            Ftlbs(u) => Joules(u * FTLB_TO_JOULE),
        }
    }
    pub fn to_ftlbs(self) -> Self {
        match self {
            u @ Ftlbs(_) => u,
            Joules(u) => Ftlbs(u * JOULE_TO_FTLB),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Pressure {
    Pascals(Numeric),
    Inhg(Numeric),
}
impl From<Pressure> for Numeric {
    fn from(u: Pressure) -> Numeric {
        match u {
            Pascals(u) => u,
            Inhg(u) => u,
        }
    }
}
impl Pressure {
    pub fn to_num(self) -> Numeric {
        From::from(self)
    }
    pub fn to_pascals(self) -> Self {
        match self {
            u @ Pascals(_) => u,
            Inhg(u) => Pascals(u * INHG_TO_PASCAL),
        }
    }
    pub fn to_inhg(self) -> Self {
        match self {
            u @ Inhg(_) => u,
            Pascals(u) => Inhg(u * PASCAL_TO_INHG),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Density {
    Kgpm3(Numeric),
    Lbpf3(Numeric),
}
impl From<Density> for Numeric {
    fn from(u: Density) -> Numeric {
        match u {
            Kgpm3(u) => u,
            Lbpf3(u) => u,
        }
    }
}
impl Density {
    pub fn to_num(self) -> Numeric {
        From::from(self)
    }
    pub fn to_kgpm3(self) -> Self {
        match self {
            u @ Kgpm3(_) => u,
            Lbpf3(u) => Kgpm3(u * LBPF3_TO_KGPM3),
        }
    }
    pub fn to_lbpf3(self) -> Self {
        match self {
            u @ Lbpf3(_) => u,
            Kgpm3(u) => Lbpf3(u * KGPM3_TO_LBPF3),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Velocity {
    Mps(Numeric),
    Mph(Numeric),
    Fps(Numeric),
}
impl From<Velocity> for Numeric {
    fn from(u: Velocity) -> Numeric {
        match u {
            Mps(u) => u,
            Mph(u) => u,
            Fps(u) => u,
        }
    }
}
impl Velocity {
    pub fn to_num(self) -> Numeric {
        From::from(self)
    }
    pub fn to_mps(self) -> Self {
        match self {
            u @ Mps(_) => u,
            Mph(u) => Mps(u * MPH_TO_MPS),
            Fps(u) => Mps(u * FPS_TO_MPS),
        }
    }
    pub fn to_mph(self) -> Self {
        match self {
            u @ Mph(_) => u,
            Mps(u) => Mph(u * MPS_TO_MPH),
            Fps(u) => Mph(u * FPS_TO_MPH),
        }
    }
    pub fn to_fps(self) -> Self {
        match self {
            u @ Fps(_) => u,
            Mps(u) => Fps(u * MPS_TO_FPS),
            Mph(u) => Fps(u * MPH_TO_FPS),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Acceleration {
    Mps2(Numeric),
    Mph2(Numeric),
    Fps2(Numeric),
}
impl From<Acceleration> for Numeric {
    fn from(u: Acceleration) -> Numeric {
        match u {
            Mps2(u) => u,
            Mph2(u) => u,
            Fps2(u) => u,
        }
    }
}
impl Acceleration {
    pub fn to_num(self) -> Numeric {
        From::from(self)
    }
    pub fn to_mps2(self) -> Self {
        match self {
            u @ Mps2(_) => u,
            Mph2(u) => Mps2(u * MPH2_TO_MPS2),
            Fps2(u) => Mps2(u * FPS2_TO_MPS2),
        }
    }
    pub fn to_mph2(self) -> Self {
        match self {
            u @ Mph2(_) => u,
            Mps2(u) => Mph2(u * MPS2_TO_MPH2),
            Fps2(u) => Mph2(u * FPS2_TO_MPH2),
        }
    }
    pub fn to_fps2(self) -> Self {
        match self {
            u @ Fps2(_) => u,
            Mps2(u) => Fps2(u * MPS2_TO_FPS2),
            Mph2(u) => Fps2(u * MPH2_TO_FPS2),
        }
    }
}
