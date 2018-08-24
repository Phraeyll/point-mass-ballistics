// extern crate ndarray;
// extern crate ndarray_linalg;
extern crate nalgebra as na;

use std::f64::consts::{E, PI};

// use self::ndarray::*;
// use self::ndarray_linalg::*;
use self::na::Vector3;


use consts::*;

#[derive(Debug)]
pub struct Projectile {
    // Constant properties
    pub m: f64, // Mass (kg)
    pub r: f64, // Radius (m)
    pub i: f64, // Form Factor (dimensionless)

    // Mutatable from ballistic calculations
    pub p: Vector3<f64>, // Position (m)
    pub v: Vector3<f64>, // Velocity (m/s)
    pub a: Vector3<f64>, // Acceleration (m/s^2)
    pub t: f64,         // Position in time (s)
}

impl Projectile {
    pub fn new(weight_grains: f64, caliber: f64, bc: f64, initial_velocity: f64) -> Self {
        let m = weight_grains * GRAINS_TO_KG;
        let r = (caliber / 2.0) * INCHES_TO_METERS;
        let i = (weight_grains * GRAINS_TO_LBS) / (caliber.powf(2.0) * bc);
        Self {
            m: m,
            r: r,
            i: i,
            a: Vector3::new(0.0, 0.0, 0.0),
            v: Vector3::new(initial_velocity * FEET_TO_METERS, 0.0, 0.0),
            p: Vector3::new(0.0, 0.0, 0.0),
            t: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct Conditions {
    pub wv: Vector3<f64>, // Wind Velocity (m/s)
    pub rho: f64,        // Density of air (kg/m^3)

    // pub ptmp: f64,       // Powder Temperature (K?)

    // pub lat:  f64,       // Lattitude (Coriolis Effect)
    // pub long: f64,       // Longitude (Coriolis Effect)
    // pub dir:  Direction, // Direction Facing (Coriolis Effect)
    // pub spin: f64,       // Spin drift (Gyroscopic Drift)
}

impl Conditions {
    pub fn new(
        temp: f64,
        wind_velocity: f64,
        wind_angle: f64,
        pressure: f64,
        humidity: f64,
    ) -> Self {
        let wv = wind_velocity * MILES_PER_HOUR_TO_METERS_PER_SECOND;
        let temp_c = (temp + F_TO_C) * F_TO_CK;
        let temp_k = (temp + F_TO_K) * F_TO_CK;
        let pa = pressure * INHG_TO_PA;
        let pv =
            humidity * 6.1121 * E.powf((18.678 - (temp_c / 234.5)) * temp_c / (257.14 + temp_c));
        Self {
            wv: Vector3::new(wv * wind_angle.cos(), 0.0, wv * wind_angle.sin()),
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

    fn delta_p(&self, f64) -> Vector3<f64>;
    fn delta_v(&self, f64) -> Vector3<f64>;
    fn a_after_drag(&self, &Conditions, f64) -> Vector3<f64>;

    fn pnorm(&self) -> f64;
    fn vnorm(&self) -> f64;
    fn anorm(&self) -> f64;

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
    fn a_after_drag(&self, c: &Conditions, cd: f64) -> Vector3<f64> {
        // Force of drag, based on specified table and current velocity (mach)
        // Coefficient of drag to be looked up from table - passed by parameter, for now, for testing
        -((c.rho * self.area() * &self.v * self.vnorm() * cd * self.i) / 2.0) / self.m
    }

    // New position
    fn delta_p(&self, t: f64) -> Vector3<f64> {
        // Not sure if second half is actually necesarry - look into differential equations
        // Intuition seems to make sense; velocity per time, then modify based on velocity
        // due to the integral of acceleration (1/2 A * t^2)
        &self.v * t // + &self.a * t.powf(2.0) / 2.0
    }

    // New velocity
    fn delta_v(&self, t: f64) -> Vector3<f64> {
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

    // Reduction of vectors to normalized values
    fn pnorm(&self) -> f64 {
        self.p.norm()
    }
    fn vnorm(&self) -> f64 {
        self.v.norm()
    }
    fn anorm(&self) -> f64 {
        self.a.norm()
    }
}
