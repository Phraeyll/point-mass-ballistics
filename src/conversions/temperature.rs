use self::Temperature::*;

pub const F_TO_CK: f64 = 5.0 / 9.0;
pub const CK_TO_F: f64 = 1.0 / F_TO_CK;

// Additive
pub const C_TO_K: f64 = 273.15;
pub const K_TO_C: f64 = -C_TO_K;

// Additive
pub const F_TO_K: f64 = 459.67;
pub const K_TO_F: f64 = -F_TO_K;

// Additive
pub const F_TO_C: f64 = -32.0;
pub const C_TO_F: f64 = -F_TO_C;

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
