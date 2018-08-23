extern crate ndarray;
extern crate ndarray_linalg;

use ::std::f64::consts::{PI, E};
use super::consts::*;
use ndarray::*;
use ndarray_linalg::*;

#[derive(Debug)]
pub struct Projectile {
    // Constant properties
    pub m: f64,          // Mass (kg)
    pub r: f64,          // Radius (m)
    pub i: f64,          // Form Factor (dimensionless)

    // Mutatable from ballistic calculations
    pub a: Array1<f64>,  // Acceleration (m/s^2)
    pub v: Array1<f64>,  // Velocity (m/s)
    pub p: Array1<f64>,  // Position (m)
    pub t: f64,          // Position in time (s)
}

impl Projectile {
    pub fn new(weight_grains: f64, caliber: f64, bc: f64, initial_velocity: f64) -> Self {
        let m = weight_grains * GRAINS_TO_KG;
        let r = (caliber / 2.0) * INCHES_TO_METERS;
        let i = (weight_grains * GRAINS_TO_LBS ) / (caliber.powf(2.0) * bc);
        Self {
            m: m,
            r: r,
            i: i,
            a: arr1(&[
                0.0,
                0.0,
                0.0,
            ]),
            v: arr1(&[
                initial_velocity * FEET_TO_METERS,
                0.0,
                0.0,
            ]),
            p: arr1(&[
                0.0,
                0.0,
                0.0,
            ]),
            t: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct Conditions {
    pub wv: Array1<f64>, // Wind Velocity (m/s)
    pub rho: f64,        // Density of air (kg/m^3)

    // pub ptmp: f64,       // Powder Temperature (K?)

    // pub lat:  f64,       // Lattitude (Coriolis Effect)
    // pub long: f64,       // Longitude (Coriolis Effect)
    // pub dir:  Direction, // Direction Facing (Coriolis Effect)
    // pub spin: f64,       // Spin drift (Gyroscopic Drift)
}

impl Conditions {
    pub fn new(temp: f64, wind_velocity: f64, wind_angle: f64, pressure: f64, humidity: f64) -> Self {
        let wv = wind_velocity * MILES_PER_HOUR_TO_METERS_PER_SECOND;
        let temp_c = (temp + F_TO_C) * F_TO_CK;
        let temp_k = (temp + F_TO_K) * F_TO_CK;
        let pa = pressure * INHG_TO_PA;
        let pv = humidity * 6.1121 * E.powf(
                (18.678 - (temp_c / 234.5)) * temp_c /
                (257.14 + temp_c)
        );
        Self {
            wv: arr1(&[
                wv * wind_angle.cos(),
                0.0,
                wv * wind_angle.sin(),
            ]),
            rho: ((pa * MOLAR_AIR) + (pv * MOLAR_WATER_VAPOR)) / (UNIVERSAL_GAS * temp_k),
        }
    }
}

pub trait Ballistic {
    fn area(&self) -> f64;
    fn caliber(&self) -> f64;
    fn weight(&self) -> f64;
    fn sd(&self) -> f64;
    fn bc(&self) -> f64;

    fn a_after_drag(&self, &Conditions, f64) -> Array1<f64>;
    fn delta_p(&self, f64) -> Array1<f64>;
    fn delta_v(&self, f64) -> Array1<f64>;

    fn step_forward(&mut self, f64, &Conditions, f64);
}

impl Ballistic for Projectile {
    fn area(&self) -> f64 {
       PI * self.r.powf(2.0)
    }

    // BC math (dependent on units)
    fn caliber(&self) -> f64 {
        self.r * METERS_TO_INCHES * 2.0
    }
    fn weight(&self) -> f64 {
        self.m * KG_TO_LBS
    }
    fn sd(&self) -> f64 {
        self.weight() / self.caliber().powf(2.0)
    }
    fn bc(&self) -> f64 {
        self.sd() / self.i
    }

    // New Acceleration (deceleration) due to drag force and gravity
    fn a_after_drag(&self, c: &Conditions, cd: f64) -> Array1<f64> {
        // Force of drag, based on specified table and current velocity (mach)
        // Coefficient of drag to be looked up from table - passed by parameter, for now, for testing
        -((c.rho * self.area() * &self.v * self.v.norm_l2() * cd * self.i) / 2.0) / self.m
    }

    // New position
    fn delta_p(&self, t: f64) -> Array1<f64> {
        // Not sure if second half is actually necesarry - look into differential equations
        // Intuition seems to make sense; velocity per time, then modify based on velocity
        // due to the integral of acceleration (1/2 A * t^2)
        &self.v * t // + &self.a * t.powf(2.0) / 2.0
    }

    // New velocity
    fn delta_v(&self, t: f64) -> Array1<f64> {
        &self.a * t
    }

    // Step forward t increments in time (seconds)
    fn step_forward(&mut self, t: f64, c: &Conditions, cd: f64) {
        self.t += t;

        self.a = self.a_after_drag(&c, cd);
        self.a[1] += GRAVITY;

        self.p = &self.p + &self.delta_p(t);

        self.v = &self.v + &self.delta_v(t);
    }
}
