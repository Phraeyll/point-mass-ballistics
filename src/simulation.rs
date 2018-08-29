use na::Vector3;

use physics::*;
use conversions::*;
use dragtables::*;

pub use dragtables::TableKind;
pub use physics::VelocityKind;

use std::f64::consts::PI;

pub struct Simulation {
    // Constant properties
    pub m: f64, // Mass (kg)
    pub r: f64, // Radius (m)
    pub i: f64, // Form Factor (dimensionless)

    // Variables for Simulation
    pub table: Table,  // Drag Function Table
    pub timestep: f64, // Timestamp for simulation (s)

    pub p: Vector3<f64>, // Position (m)
    pub v: Vector3<f64>, // Velocity (m/s)
    pub a: Vector3<f64>, // Acceleration (m/s^2)
    pub t: f64,          // Position in time (s)

    // Environmental Conditions
    pub wv: Vector3<f64>, // Wind Velocity (m/s)
    pub rho: f64,         // Density of air (kg/m^3)
    pub c: f64,           // Speed of sound (m/s)
    pub g: Vector3<f64>,  // Gravity (m/s^2)

    // Other factors, not calculated yet
    // pub ptmp: f64,       // Powder Temperature (K?)
    // pub lat:  f64,       // Lattitude (Coriolis Effect)
    // pub long: f64,       // Longitude (Coriolis Effect)
    // pub dir:  Direction, // Direction Facing (Coriolis Effect)
    // pub spin: f64,       // Spin drift (Gyroscopic Drift)
}

pub trait Projectile {
    fn area(&self) -> f64;
    fn caliber(&self) -> f64;
    fn weight(&self) -> f64;
    fn sd(&self) -> f64;
    fn bc(&self) -> f64;
}

pub trait Normalize {
    fn pnorm(&self) -> f64;
    fn vnorm(&self) -> f64;
    fn anorm(&self) -> f64;
}

pub trait Drag {
    fn a_after_drag(&self) -> Vector3<f64>;
    fn cd(&self) -> f64;
}

pub trait Output {
    fn time(&self) -> f64;
    fn velocity(&self) -> f64;
    fn mach(&self) -> f64;
    fn distance(&self) -> f64;
    fn drop(&self) -> f64;
    fn windage(&self) -> f64;
}

impl Simulation {
    pub fn new(
        weight_grains: f64,
        caliber: f64,
        bc: f64,
        initial_velocity: f64,
        launch_angle: f64,
        drag_table: TableKind,
        timestep: f64,
        wind_velocity: f64,
        wind_angle: f64,
        temp: f64,
        pressure: f64,
        humidity: f64,
    ) -> Self {
        let m = mass(weight_grains);
        let r = radius(caliber);
        let i = form_factor(weight_grains, caliber, bc);

        let iv = velocity_tuple(Projectile(initial_velocity), launch_angle);
        let wv = velocity_tuple(Wind(wind_velocity), wind_angle);
        let g = gravity();
        let rho = air_density(temp, humidity, pressure);
        let c = speed_sound(rho, pressure);

        let table = Table::new(drag_table);

        Self {
            m,
            r,
            i,
            table,
            timestep,
            rho,
            c,
            p: Vector3::new(0.0, 0.0, 0.0),
            v: Vector3::new(iv.0, iv.1, iv.2),
            a: Vector3::new(0.0, 0.0, 0.0),
            wv: Vector3::new(wv.0, wv.1, wv.2),
            g: Vector3::new(g.0, g.1, g.2),
            t: 0.0,
        }
    }
}

impl Projectile for Simulation {
    fn area(&self) -> f64 {
        PI * self.r.powf(2.0)
    }
    fn caliber(&self) -> f64 {
        self.r * METERS_TO_INCHES * 2.0
    }
    fn weight(&self) -> f64 {
        self.m * KGS_TO_LBS
    }
    fn sd(&self) -> f64 {
        self.weight() / self.caliber().powf(2.0)
    }
    fn bc(&self) -> f64 {
        self.sd() / self.i
    }
}

impl Normalize for Simulation {
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

impl Output for Simulation {
    fn time(&self) -> f64 {
        self.t
    }
    fn velocity(&self) -> f64 {
        self.vnorm() * MPS_TO_FPS
    }
    fn mach(&self) -> f64 {
        self.vnorm() / self.c
    }
    fn distance(&self) -> f64 {
        self.p.x * METERS_TO_YARDS
    }
    fn drop(&self) -> f64 {
        self.p.y * METERS_TO_INCHES
    }
    fn windage(&self) -> f64 {
        self.p.z * METERS_TO_INCHES
    }
}

impl Drag for Simulation {
    fn a_after_drag(&self) -> Vector3<f64> {
        let cd = (self.rho * self.area() * self.cd() * self.i) / (2.0 * self.m);
        let vv = self.v - self.wv;
        -cd * vv.norm() * vv + self.g
    }
    fn cd(&self) -> f64 {
        let mach = self.mach();
        let (x, y) = self.table.find(mach);
        y.0 + (mach - x.0) * (y.1 - y.0) / (x.1 - x.0)
    }
}

impl Iterator for Simulation {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        self.a = self.a_after_drag();
        self.p = self.p + self.v * self.timestep + self.a * (self.timestep.powf(2.0) / 2.0);
        self.v = self.v + self.a * self.timestep;
        self.t += self.timestep;
        Some(self.distance())
    }
}
