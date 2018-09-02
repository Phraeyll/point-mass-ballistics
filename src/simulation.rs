use na::{Vector3, Rotation3};

pub use dragtables::DragTableKind;

use self::constructors::*;
use conversions::consts::*;
use conversions::*;
use dragtables::*;

use std::f64::consts::{PI, E};

pub const GRAVITY: Acceleration = Acceleration::Mps2(-9.80665); // Local gravity in m/s
pub const UNIVERSAL_GAS: f64 = 8.314; // Universal gas constant (J/K*mol)
pub const MOLAR_DRY: f64 = 0.0289644; // Molar mass of dry air (kg/mol)
pub const MOLAR_VAPOR: f64 = 0.018016; // Molar mass of water vapor (kg/mol)

pub struct PointMassModel {
    // Constant properties
    pub weight: WeightMass,   // Weight (grains)
    pub caliber: Length, // Caliber (inches)
    pub bc: f64,      // Ballistic Coefficient

    // Envelope of motion
    pub position: Vector3<f64>,     // Position (m)
    pub velocity: Vector3<f64>,     // Velocity (m/s)
    pub acceleration: Vector3<f64>, // Acceleration (m/s^2)
    pub time: f64,                  // Position in time (s)

    // Variables for simulation
    pub drag_table: DragTable,       // Drag Function DragTable
    pub time_step: f64,              // Timestep for simulation (s)
    pub launch_angle: f64,           // Initial launch angle (degrees)
    pub scope_height: Length,        // Scope Height (inches)
    pub zero_distance: Length,

    // Environmental Conditions
    pub wind_velocity: Vector3<f64>, // Wind Velocity (m/s)
    pub temperature: Temperature,    // Temperature (F)
    pub pressure: Pressure,          // Pressure (InHg)
    pub humidity: f64,               // Humidity (0-1)
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
    fn area(&self) -> f64; // Area (meters)
    fn mass(&self) -> f64; // Mass (kgs)
    fn radius(&self) -> f64; // Radius (meters)
    fn sd(&self) -> f64; // Sectional Density
    fn i(&self) -> f64; // Form Factor
}
pub trait DragSimulation {
    fn rho(&self) -> f64;  // Density of air (kg/m^3)
    fn mach(&self) -> f64; // Velocity rel ative to speed of sound
    fn drag_force(&self) -> Vector3<f64>;
}

pub trait Output {
    fn time(&self) -> f64;
    fn velocity(&self) -> f64;
    fn distance(&self) -> f64;
    fn drop(&self) -> f64;
    fn windage(&self) -> f64;
    fn angle(&self) -> f64;
    fn relative_velocity(&self) -> f64;
    fn relative_distance(&self) -> f64;
    fn relative_drop(&self) -> f64;
    fn relative_windage(&self) -> f64;
}


impl PointMassModel {
    pub fn new(
        weight: f64,
        caliber: f64,
        bc: f64,
        initial_velocity: f64,
        launch_angle: f64,
        scope_height: f64,
        zero_distance: f64,
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
        let time_step_seconds = Time::Seconds(time_step);
        let scope_height_inches = Length::Inches(scope_height);
        let zero_distance_yards = Length::Yards(zero_distance);

        Self {
            weight: weight_grains,
            caliber: diameter_inches,
            bc,

            position: Vector3::new(
                    ZERO_METERS.into(),
                    ZERO_METERS.into(),
                    ZERO_METERS.into()
            ),
            velocity: construct_velocity(initial_velocity_fps, Projectile(launch_angle)),
            acceleration: Vector3::new(
                    ZERO_MPS2.into(),
                    ZERO_MPS2.into(),
                    ZERO_MPS2.into()
            ),
            time: ZERO_SECONDS.into(),

            drag_table: DragTable::new(drag_table),
            time_step: time_step_seconds.to_seconds().into(),
            launch_angle,
            scope_height: scope_height_inches,
            zero_distance: zero_distance_yards,

            wind_velocity: construct_velocity(wind_velocity_mph, Wind(wind_angle)),
            temperature: temperature_f,
            pressure: pressure_inhg,
            humidity,
            g: Vector3::new(
                ZERO_MPS2.into(),
                GRAVITY.into(),
                ZERO_MPS2.into(),
            ),
        }
    }
}

impl Projectile for PointMassModel {
    fn area(&self) -> f64 {
        PI * self.radius().powf(2.0)
    }
    fn mass(&self) -> f64 {
        self.weight.to_kgs().into()
    }
    fn radius(&self) -> f64 {
        f64::from(self.caliber.to_meters()) / 2.0
    }
    fn sd(&self) -> f64 {
        f64::from(self.weight.to_lbs()) / f64::from(self.caliber.to_inches()).powf(2.0)
    }
    fn i(&self) -> f64 {
        self.sd() / self.bc
    }

}

impl Output for PointMassModel {
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
    fn angle(&self) -> f64 {
        self.launch_angle.to_radians()
    }
    fn relative_velocity(&self) -> f64 {
        let axis = Vector3::z_axis();
        let rotation = Rotation3::from_axis_angle(&axis, -self.angle());
        let velocity = rotation * self.velocity;
        f64::from(Velocity::Mps(velocity.norm()).to_fps())
    }
    fn relative_distance(&self) -> f64 {
        let axis = Vector3::z_axis();
        let rotation = Rotation3::from_axis_angle(&axis, -self.angle());
        let position = rotation * self.position;
        f64::from(Length::Meters(position.x).to_yards())
    }
    fn relative_drop(&self) -> f64 {
        let axis = Vector3::z_axis();
        let rotation = Rotation3::from_axis_angle(&axis, -self.angle());
        let position = rotation * self.position;
        let drop = f64::from(Length::Meters(position.y)) - f64::from(self.scope_height.to_meters());
        f64::from(Length::Meters(drop).to_inches())
    }
    fn relative_windage(&self) -> f64 {
        let axis = Vector3::z_axis();
        let rotation = Rotation3::from_axis_angle(&axis, -self.angle());
        let position = rotation * self.position;
        f64::from(Length::Meters(position.z).to_inches())
    }
}

impl DragSimulation for PointMassModel {
    fn rho(&self) -> f64 {
        let celsius = f64::from(self.temperature.to_celsius());
        let kelvin = f64::from(self.temperature.to_kelvin());
        let pa = f64::from(self.pressure.to_pascals());
        let pv =
            self.humidity * 611.21 * E.powf((18.678 - (celsius / 234.5)) * (celsius / (257.14 + celsius)));
        let pd = pa - pv;
        ((pd * MOLAR_DRY) + (pv * MOLAR_VAPOR)) / (UNIVERSAL_GAS * kelvin)
    }
    fn mach(&self) -> f64 {
        let pa = f64::from(self.pressure.to_pascals());
        let c = (1.4 * (pa / self.rho())).sqrt();
        self.velocity.norm() / c
    }
    fn drag_force(&self) -> Vector3<f64> {
        let cd = self.drag_table.lerp(self.mach()) * self.i();
        let vv = self.velocity - self.wind_velocity;
        -(self.rho() * self.area() * vv * vv.norm() * cd) / 2.0
    }
}

impl Iterator for PointMassModel {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        self.acceleration = self.drag_force() / self.mass() + self.g;
        self.position = self.position
            + self.velocity * self.time_step
            + self.acceleration * (self.time_step.powf(2.0) / 2.0);
        self.velocity = self.velocity + self.acceleration * self.time_step;
        self.time += self.time_step;
        Some(self.relative_distance())
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
}
