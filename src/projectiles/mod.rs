extern crate nalgebra as na;

use self::na::Vector3;

use cdtables::*;
use consts::*;

use std::f64::consts::{E, PI};

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
    pub t: f64,          // Position in time (s)

    // Environmental Conditions
    pub wv: Vector3<f64>, // Wind Velocity (m/s)
    pub rho: f64,         // Density of air (kg/m^3)
    pub c: f64,           // Speed of sound (m/s)
    pub g: Vector3<f64>,  // Gravity (m/s^2)

    // Other factors, not caldulated yet
    // pub ptmp: f64,       // Powder Temperature (K?)
    // pub lat:  f64,       // Lattitude (Coriolis Effect)
    // pub long: f64,       // Longitude (Coriolis Effect)
    // pub dir:  Direction, // Direction Facing (Coriolis Effect)
    // pub spin: f64,       // Spin drift (Gyroscopic Drift)
}

impl Projectile {
    pub fn new(
        weight_grains: f64,
        caliber: f64,
        bc: f64,
        initial_velocity: f64,
        wind_velocity: f64,
        wind_angle: f64,
        temp: f64,
        pressure: f64,
        humidity: f64,
    ) -> Self {
        let m = weight_grains * GRAINS_TO_KG;
        let r = (caliber / 2.0) * INCHES_TO_METERS;
        let i = (weight_grains * GRAINS_TO_LBS) / (caliber.powf(2.0) * bc);

        let a = Vector3::new(0.0, 0.0, 0.0);
        let v = Vector3::new(initial_velocity * FEET_TO_METERS, 0.0, 0.0);
        let p = Vector3::new(0.0, 0.0, 0.0);
        let t = 0.0;

        let wind = wind_velocity * MILES_PER_HOUR_TO_METERS_PER_SECOND;
        let wv = Vector3::new(wind * wind_angle.cos(), 0.0, wind * wind_angle.sin());

        let temp_c = (temp + F_TO_C) * F_TO_CK;
        let temp_k = (temp + F_TO_K) * F_TO_CK;
        let pa = pressure * INHG_TO_PA;
        let pv =
            humidity * 6.1121 * E.powf((18.678 - (temp_c / 234.5)) * temp_c / (257.14 + temp_c));
        let rho = ((pa * MOLAR_AIR) + (pv * MOLAR_WATER_VAPOR)) / (UNIVERSAL_GAS * temp_k);

        let c = (1.4 * pa / rho).sqrt();
        let g = Vector3::new(0.0, GRAVITY, 0.0);

        Self {
            m,
            r,
            i,

            a,
            v,
            p,
            t,

            wv,
            rho,
            c,
            g,
        }
    }
}

pub trait Ballistic {
    fn area(&self) -> f64;
    fn caliber(&self) -> f64;
    fn weight(&self) -> f64;
    fn sd(&self) -> f64;
    fn bc(&self) -> f64;
    fn pnorm(&self) -> f64;
    fn vnorm(&self) -> f64;
    fn anorm(&self) -> f64;

    fn cd(&self, &Table) -> f64;
    fn a_after_drag(&self, &Table) -> Vector3<f64>;
    fn step_forward(&mut self, &Table, f64);
}

impl Ballistic for Projectile {
    fn area(&self) -> f64 {
        PI * self.r.powf(2.0)
    }
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
    fn pnorm(&self) -> f64 {
        self.p.norm()
    }
    fn vnorm(&self) -> f64 {
        self.v.norm()
    }
    fn anorm(&self) -> f64 {
        self.a.norm()
    }

    fn cd(&self, table: &Table) -> f64 {
        let x = self.vnorm() / self.c;
        let mut cd = 0.0; // beter defaults?
        let mut x0 = 0.0;
        let mut y0 = 0.0;
        for (k, v) in table.0.iter() {
            let (x1, y1) = (k.0, *v);
            if x1 == x {
                cd = y1;
                break;
            } else if x1 > x {
                cd = y0 + (x - x0) * (y1 - y0) / (x1 - x0);
                break;
            }
            x0 = x1;
            y0 = y1;
        }
        cd
    }

    fn a_after_drag(&self, table: &Table) -> Vector3<f64> {
        let cd = (self.rho * self.area() * self.cd(&table) * self.i) / (2.0 * self.m);
        let vv = self.v - self.wv;
        -cd * self.vnorm() * vv + self.g
    }

    fn step_forward(&mut self, table: &Table, timestep: f64) {
        self.a = self.a_after_drag(&table);
        self.p = self.p + self.v * timestep + self.a * (timestep.powf(2.0) / 2.0);
        self.v = self.v + self.a * timestep;

        self.t += timestep;
    }
}
