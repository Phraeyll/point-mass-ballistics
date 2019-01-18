use Temperature::*;
use crate::util::Numeric;

pub const F_TO_CK: Numeric = 5.0 / 9.0;
pub const CK_TO_F: Numeric = 1.0 / F_TO_CK;

// Additive
pub const C_TO_K: Numeric = 273.15;
pub const K_TO_C: Numeric = -C_TO_K;

// Additive
pub const F_TO_K: Numeric = 459.67;
pub const K_TO_F: Numeric = -F_TO_K;

// Additive
pub const F_TO_C: Numeric = -32.0;
pub const C_TO_F: Numeric = -F_TO_C;

#[derive(Debug, Copy, Clone)]
pub enum Temperature {
    C(Numeric),
    K(Numeric),
    F(Numeric),
}
impl From<Temperature> for Numeric {
    fn from(u: Temperature) -> Numeric {
        match u {
            C(u) => u,
            K(u) => u,
            F(u) => u,
        }
    }
}
impl self::Temperature {
    pub fn to_num(self) -> Numeric {
        Numeric::from(self)
    }
    pub fn to_celsius(self) -> Self {
        match self {
            u @ C(_) => u,
            K(u) => C(u + K_TO_C),
            F(u) => C((u + F_TO_C) * F_TO_CK),
        }
    }
    pub fn to_kelvin(self) -> Self {
        match self {
            u @ K(_) => u,
            C(u) => K(u + C_TO_K),
            F(u) => K((u + F_TO_K) * F_TO_CK),
        }
    }
    pub fn to_fahrenheit(self) -> Self {
        match self {
            u @ F(_) => u,
            C(u) => F((u * CK_TO_F) + C_TO_F),
            K(u) => F((u * CK_TO_F) + K_TO_F),
        }
    }
}
