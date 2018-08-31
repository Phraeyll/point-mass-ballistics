use self::Temperature::*;
use super::consts::*;

#[derive(Copy, Clone)]
pub enum Temperature {
    C(f64),
    K(f64),
    F(f64),
}
impl From<Temperature> for f64 {
    fn from(u: Temperature) -> f64 {
        match u {
            C(u) => u,
            K(u) => u,
            F(u) => u,
        }
    }
}
impl self::Temperature {
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
