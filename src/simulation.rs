use na::Vector3;

use conversions::*;
use dragtables::*;
use physics::*;

pub use dragtables::TableKind;
pub use physics::VelocityKind;

pub struct Simulation {
    // Constant properties
    pub mass: f64,   // Mass (kg)
    pub radius: f64, // Radius (m)
    pub area: f64,   // Area (m)
    pub caliber: f64,// Caliber (inch)
    pub weight: f64, // Weight (grain)
    pub sd: f64,     // Sectional Density
    pub bc: f64,     // Ballistic Coefficient
    pub i: f64,      // Form Factor

    // Envelope of motion
    pub position: Vector3<f64>,     // Position (m)
    pub velocity: Vector3<f64>,     // Velocity (m/s)
    pub acceleration: Vector3<f64>, // Acceleration (m/s^2)
    pub time: f64,                  // Position in time (s)

    // Variables for simulation
    pub table: Table,  // Drag Function Table
    pub time_step: f64, // Timestep for simulation (s)

    // Environmental Conditions
    pub wind_velocity: Vector3<f64>, // Wind Velocity (m/s)
    pub rho: f64,                    // Density of air (kg/m^3)
    pub c: f64,                      // Speed of sound (m/s)
    pub g: Vector3<f64>,             // Gravity (m/s^2)
    // Other factors, not calculated yet
    // pub ptmp: f64,       // Powder Temperature (K?)
    // pub lat:  f64,       // Lattitude (Coriolis Effect)
    // pub long: f64,       // Longitude (Coriolis Effect)
    // pub dir:  Direction, // Direction Facing (Coriolis Effect)
    // pub spin: f64,       // Spin drift (Gyroscopic Drift)
}

pub trait Drag {
    fn acceleration_from_drag(&self) -> Vector3<f64>;
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
        weight: f64,
        caliber: f64,
        bc: f64,
        initial_velocity: f64,
        launch_angle: f64,
        drag_table: TableKind,
        time_step: f64,
        wind_velocity: f64,
        wind_angle: f64,
        temperature: f64,
        pressure: f64,
        humidity: f64,
    ) -> Self {
        let mass = mass(weight);
        let radius = radius(caliber);
        let area = area(radius);
        let sd = weight / caliber.powf(2.0);
        let i = form_factor(weight, caliber, bc);
        let iv = velocity_tuple(Projectile(initial_velocity), launch_angle);
        let table = Table::new(drag_table);
        let wv = velocity_tuple(Wind(wind_velocity), wind_angle);
        let rho = air_density(temperature, humidity, pressure);
        let c = speed_sound(rho, pressure);
        let g = gravity();

        Self {
            mass,
            radius,
            area,
            caliber,
            weight,
            sd,
            bc,
            i,

            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(iv.0, iv.1, iv.2),
            acceleration: Vector3::new(0.0, 0.0, 0.0),
            time: 0.0,

            table,
            time_step,

            wind_velocity: Vector3::new(wv.0, wv.1, wv.2),
            rho,
            c,
            g: Vector3::new(g.0, g.1, g.2),
        }
    }
}

impl Output for Simulation {
    fn time(&self) -> f64 {
        f64::from(Time::Seconds(self.time).to_seconds())
    }
    fn velocity(&self) -> f64 {
        f64::from(Velocity::Mps(self.velocity.norm()).to_fps())
    }
    fn mach(&self) -> f64 {
        self.velocity.norm() / self.c
    }
    fn distance(&self) -> f64 {
        f64::from(Length::Meters(self.position.x).to_yards())
    }
    fn drop(&self) -> f64 {
        f64::from(Length::Meters(self.position.y).to_inches())
    }
    fn windage(&self) -> f64 {
        f64::from(Length::Meters(self.position.z).to_inches())
    }
}

impl Drag for Simulation {
    fn acceleration_from_drag(&self) -> Vector3<f64> {
        let cd = self.table.lerp(self.mach());
        let cdv = (self.rho * self.area * cd * self.i) / (2.0 * self.mass);
        let vv = self.velocity - self.wind_velocity;
        -cdv * vv.norm() * vv + self.g
    }
}

impl Iterator for Simulation {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        self.acceleration = self.acceleration_from_drag();
        self.position = self.position + self.velocity * self.time_step + self.acceleration * (self.time_step.powf(2.0) / 2.0);
        self.velocity = self.velocity + self.acceleration * self.time_step;
        self.time += self.time_step;
        Some(self.distance())
    }
}
