use consts::*;

pub fn f_to_c(temp: f64) -> f64 {
    (temp + F_TO_C) * F_TO_CK
}

pub fn f_to_k(temp: f64) -> f64 {
    (temp + F_TO_K) * F_TO_CK
}
