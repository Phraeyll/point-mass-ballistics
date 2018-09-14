use self::{Acceleration::*, Density::*, Pressure::*, Velocity::*, Energy::*};
use super::{length::*, time::*, weight_mass::*};

//Energy
pub const JOULE_TO_FTLB: f64 = 0.737_56;
pub const FTLB_TO_JOULE: f64 = 1.0 / JOULE_TO_FTLB;

// Pressure
pub const INHG_TO_PASCAL: f64 = 3_386.38;
pub const PASCAL_TO_INHG: f64 = 1.0 / INHG_TO_PASCAL;

// Density
pub const LBPF3_TO_KGPM3: f64 = LBS_TO_KGS / (FEET_TO_METERS * FEET_TO_METERS * FEET_TO_METERS);
pub const KGPM3_TO_LBPF3: f64 = 1.0 / LBPF3_TO_KGPM3;

// Velocity
pub const MPH_TO_MPS: f64 = MPH_TO_FPS * FPS_TO_MPS;
pub const MPS_TO_MPH: f64 = 1.0 / MPH_TO_MPS;

pub const MPH_TO_FPS: f64 = MILES_TO_FEET / HOURS_TO_SECONDS;
pub const FPS_TO_MPH: f64 = 1.0 / MPH_TO_FPS;

pub const FPS_TO_MPS: f64 = FEET_TO_METERS;
pub const MPS_TO_FPS: f64 = 1.0 / FPS_TO_MPS;

// Acceleration ??
pub const MPH2_TO_MPS2: f64 = MPH2_TO_FPS2 * FPS2_TO_MPS2;
pub const MPS2_TO_MPH2: f64 = 1.0 / MPH2_TO_MPS2;

pub const MPH2_TO_FPS2: f64 = MPH_TO_FPS / HOURS_TO_SECONDS;
pub const FPS2_TO_MPH2: f64 = 1.0 / MPH2_TO_FPS2;

pub const FPS2_TO_MPS2: f64 = FPS_TO_MPS;
pub const MPS2_TO_FPS2: f64 = 1.0 / FPS2_TO_MPS2;

#[derive(Debug, Copy, Clone)]
pub enum Energy {
    Joules(f64),
    Ftlbs(f64),
}
impl From<Energy> for f64 {
    fn from(u: Energy) -> f64 {
        match u {
            Joules(u) => u,
            Ftlbs(u) => u,
        }
    }
}
impl Energy {
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
    Pascals(f64),
    Inhg(f64),
}
impl From<Pressure> for f64 {
    fn from(u: Pressure) -> f64 {
        match u {
            Pascals(u) => u,
            Inhg(u) => u,
        }
    }
}
impl Pressure {
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
    Kgpm3(f64),
    Lbpf3(f64),
}
impl From<Density> for f64 {
    fn from(u: Density) -> f64 {
        match u {
            Kgpm3(u) => u,
            Lbpf3(u) => u,
        }
    }
}
impl self::Density {
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
    Mps(f64),
    Mph(f64),
    Fps(f64),
}
impl From<Velocity> for f64 {
    fn from(u: Velocity) -> f64 {
        match u {
            Mps(u) => u,
            Mph(u) => u,
            Fps(u) => u,
        }
    }
}
impl self::Velocity {
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
    Mps2(f64),
    Mph2(f64),
    Fps2(f64),
}
impl From<Acceleration> for f64 {
    fn from(u: Acceleration) -> f64 {
        match u {
            Mps2(u) => u,
            Mph2(u) => u,
            Fps2(u) => u,
        }
    }
}
impl self::Acceleration {
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
