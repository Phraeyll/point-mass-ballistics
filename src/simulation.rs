use na::Vector3;

pub use dragtables::DragTableKind;

use self::constructors::*;
use conversions::consts::*;
use conversions::*;
use dragtables::*;
use physics;

use std::f64::consts::PI;

pub struct Simulation {
    // Constant properties
    pub mass: f64,   // Mass (kg)
    pub radius: f64, // Radius (m)
    pub i: f64,      // Form Factor

    // Envelope of motion
    pub position: Vector3<f64>,     // Position (m)
    pub velocity: Vector3<f64>,     // Velocity (m/s)
    pub acceleration: Vector3<f64>, // Acceleration (m/s^2)
    pub time: f64,                  // Position in time (s)

    // Variables for simulation
    pub drag_table: DragTable, // Drag Function DragTable
    pub time_step: f64,   // Timestep for simulation (s)

    // Environmental Conditions
    pub wind_velocity: Vector3<f64>, // Wind Velocity (m/s)
    pub rho: f64,                    // Density of air (kg/m^3)
    pub c: f64,                      // Speed of sound (m/s)
    pub g: Vector3<f64>,             // Gravity (m/s^2)
    /*
    Other factors, not calculated yet
    pub ptmp: f64,                   // Powder Temperature (K?)
    pub lat:  f64,                   // Lattitude (Coriolis Effect)
    pub long: f64,                   // Longitude (Coriolis Effect)
    pub dir:  Direction,             // Direction Facing (Coriolis Effect)
    pub spin: f64,                   // Spin drift (Gyroscopic Drift)
    */
}

pub trait Projectile {
    fn area(&self) -> f64; // Area (m)
    fn caliber(&self) -> f64; // Caliber (inch)
    fn weight(&self) -> f64; // Weight (grain)
    fn sd(&self) -> f64; // Sectional Density
    fn bc(&self) -> f64; // Ballistic Coefficient
    fn mach(&self) -> f64; // Velocity rel ative to speed of sound
}

pub trait Output {
    fn time(&self) -> f64;
    fn velocity(&self) -> f64;
    fn distance(&self) -> f64;
    fn drop(&self) -> f64;
    fn windage(&self) -> f64;
}

pub trait Drag {
    fn acceleration_from_drag(&self) -> Vector3<f64>;
}

impl Simulation {
    pub fn new(
        weight: f64,
        caliber: f64,
        bc: f64,
        initial_velocity: f64,
        launch_angle: f64,
        drag_table: DragTableKind,
        time_step: f64,
        wind_velocity: f64,
        wind_angle: f64,
        temperature: f64,
        pressure: f64,
        humidity: f64,
    ) -> Self {
        let weight_grains = WeightMass::Grains(weight);
        let diameter_inches = Length::Inches(caliber);
        let initial_velocity_fps = Velocity::Fps(initial_velocity);
        let temperature_f = Temperature::F(temperature);
        let pressure_inhg = Pressure::Inhg(pressure);
        let wind_velocity_mph = Velocity::Mph(wind_velocity);
        let rho = physics::air_density(temperature_f, pressure_inhg, humidity);
        let time_step_seconds = Time::Seconds(time_step);

        Self {
            mass: weight_grains.to_kgs().into(),
            radius: f64::from(diameter_inches.to_meters()) / 2.0,
            i: form_factor(weight_grains, diameter_inches, bc),

            position: Vector3::new(ZERO_METERS.into(), ZERO_METERS.into(), ZERO_METERS.into()),
            velocity: construct_velocity(initial_velocity_fps, Projectile(launch_angle)),
            acceleration: Vector3::new(ZERO_MPS2.into(), ZERO_MPS2.into(), ZERO_MPS2.into()),
            time: ZERO_SECONDS.into(),

            drag_table: DragTable::new(drag_table),
            time_step: time_step_seconds.to_seconds().into(),

            wind_velocity: construct_velocity(wind_velocity_mph, Wind(wind_angle)),
            rho: rho,
            c: physics::speed_sound(rho, pressure_inhg),
            g: Vector3::new(
                ZERO_MPS2.into(),
                physics::gravity().into(),
                ZERO_MPS2.into(),
            ),
        }
    }
}

impl Projectile for Simulation {
    fn area(&self) -> f64 {
        PI * self.radius.powf(2.0)
    }
    fn caliber(&self) -> f64 {
        f64::from(Length::Meters(self.radius * 2.0).to_inches())
    }
    fn weight(&self) -> f64 {
        f64::from(WeightMass::Kgs(self.mass).to_lbs())
    }
    fn sd(&self) -> f64 {
        self.weight() / self.caliber().powf(2.0)
    }
    fn bc(&self) -> f64 {
        self.sd() / self.i
    }
    fn mach(&self) -> f64 {
        self.velocity.norm() / self.c
    }
}

impl Output for Simulation {
    fn time(&self) -> f64 {
        f64::from(Time::Seconds(self.time).to_seconds())
    }
    fn velocity(&self) -> f64 {
        f64::from(Velocity::Mps(self.velocity.norm()).to_fps())
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
        let cdi = self.drag_table.lerp(self.mach()) * self.i;
        let cd = (self.rho * self.area() * cdi) / (2.0 * self.mass);
        let vv = self.velocity - self.wind_velocity;
        -cd * vv.norm() * vv + self.g
    }
}

impl Iterator for Simulation {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        self.acceleration = self.acceleration_from_drag();
        self.position = self.position
            + self.velocity * self.time_step
            + self.acceleration * (self.time_step.powf(2.0) / 2.0);
        self.velocity = self.velocity + self.acceleration * self.time_step;
        self.time += self.time_step;
        Some(self.distance())
    }
}

mod constructors {
    pub use self::AngleKind::*;

    use conversions::consts::*;
    use conversions::*;
    use na::{Rotation3, Vector3};

    pub enum AngleKind {
        Projectile(f64),
        Wind(f64),
    }

    pub fn construct_velocity(vel: Velocity, vk: AngleKind) -> Vector3<f64> {
        let (axis, angle) = match vk {
            Projectile(deg) => {
                // Rotation along z axis is pitch, projectile up/down relative to x/y plane
                (Vector3::z_axis(), deg.to_radians())
            }
            Wind(deg) => {
                // Rotation along y axis is yaw, wind left/right relative to x/z plane
                (Vector3::y_axis(), deg.to_radians())
            }
        };
        let velocity_mps = vel.to_mps().into();
        let rotation = Rotation3::from_axis_angle(&axis, angle);
        let velocity = Vector3::new(velocity_mps, ZERO_MPS.into(), ZERO_MPS.into());
        rotation * velocity
    }

    pub fn form_factor(weight: WeightMass, caliber: Length, bc: f64) -> f64 {
        f64::from(weight.to_lbs()) / (f64::from(caliber.to_inches()).powf(2.0) * bc)
    }

}
