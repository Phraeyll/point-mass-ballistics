use super::consts::*;
use self::{Density::*, Pressure::*, Velocity::*, Acceleration::*};

#[derive(Copy, Clone)]
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
            Inhg(u) => Pascals(u * INHG_TO_PASCALS),
        }
    }
    pub fn to_inhg(self) -> Self {
        match self {
            u @ Inhg(_) => u,
            Pascals(u) => Inhg(u * PASCALS_TO_INHG),
        }
    }
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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
