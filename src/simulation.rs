use na::{Rotation3, Vector3};

pub use dragtables::DragTableKind;

use self::constructors::*;
use conversions::consts::*;
use conversions::*;
use dragtables::*;

use std::f64::consts::{E, PI};

const GRAVITY: Acceleration = Acceleration::Mps2(-9.80665); // Local gravity in m/s
const UNIVERSAL_GAS: f64 = 8.314; // Universal gas constant (J/K*mol)
const MOLAR_DRY: f64 = 0.0289644; // Molar mass of dry air (kg/mol)
const MOLAR_VAPOR: f64 = 0.018016; // Molar mass of water vapor (kg/mol)

#[derive(Debug)]
pub struct PointMassModel {
    // Projectile properties
    pub weight: WeightMass, // Weight (grains)
    pub caliber: Length,    // Caliber (inches)
    pub bc: f64,            // Ballistic Coefficient

    // Environmental Conditions
    pub wind_velocity: Vector3<f64>, // Wind Velocity (m/s)
    pub temperature: Temperature,    // Temperature (F)
    pub pressure: Pressure,          // Pressure (InHg)
    pub humidity: f64,               // Humidity (0-1)
    pub g: Vector3<f64>,             // Gravity (m/s^2)

    // Variables for simulation
    pub time_step: f64,             // Timestep for simulation (s)
    pub initial_angle: f64,         // Initial launch angle (degrees)
    pub initial_velocity: Velocity, // Initial velocity (ft/s)
    pub scope_height: Length,       // Scope Height (inches)
    pub zero_distance: Length,      // Zero distance (yards)
    pub drag_table: DragTable,      // Drag Function DragTable

    /*
    Other factors, not calculated yet
    pub ptmp: f64,                   // Powder Temperature (K?)
    pub lat:  f64,                   // Lattitude (Coriolis Effect)
    pub long: f64,                   // Longitude (Coriolis Effect)
    pub dir:  Direction,             // Direction Facing (Coriolis Effect)
    pub spin: f64,                   // Spin drift (Gyroscopic Drift)
    */
}
struct Envelope {
    // Envelope of motion
    pub position: Vector3<f64>,     // Position (m)
    pub velocity: Vector3<f64>,     // Velocity (m/s)
    pub acceleration: Vector3<f64>, // Acceleration (m/s^2)
    pub time: f64,                  // Position in time (s)
}
pub struct IterPointMassModel<'a> {
    model: &'a PointMassModel,
    envelope: Envelope,
}

trait Projectile {
    fn area(&self) -> f64; // Area (meters)
    fn mass(&self) -> f64; // Mass (kgs)
    fn radius(&self) -> f64; // Radius (meters)
    fn sd(&self) -> f64; // Sectional Density
    fn i(&self) -> f64; // Form Factor
}
trait DragSimulation {
    fn rho(&self) -> f64; // Density of air (kg/m^3)
    fn mach(&self) -> f64; // Velocity rel ative to speed of sound
    fn drag_force(&self) -> Vector3<f64>;
}
pub trait Output {
    fn time(&self) -> f64;
    fn velocity(&self) -> f64;
    fn distance(&self) -> f64;
    fn drop(&self) -> f64;
    fn windage(&self) -> f64;
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
        initial_angle: f64,
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

            wind_velocity: construct_velocity(wind_velocity_mph, Wind(wind_angle.to_radians())),
            temperature: temperature_f,
            pressure: pressure_inhg,
            humidity,
            g: Vector3::new(ZERO_MPS2.into(), GRAVITY.into(), ZERO_MPS2.into()),

            time_step: time_step_seconds.to_seconds().into(),
            initial_angle,
            initial_velocity: initial_velocity_fps,
            scope_height: scope_height_inches,
            zero_distance: zero_distance_yards,
            drag_table: DragTable::new(drag_table),
        }
    }
    pub fn iter<'a>(&'a self) -> IterPointMassModel {
        let initial_angle_radians = self.initial_angle.to_radians();
        let initial_velocity_fps = self.initial_velocity;
        IterPointMassModel {
            model: self,
            envelope: Envelope {
                position: Vector3::new(ZERO_METERS.into(), ZERO_METERS.into(), ZERO_METERS.into()),
                velocity: construct_velocity(
                    initial_velocity_fps,
                    Projectile(initial_angle_radians),
                ),
                acceleration: Vector3::new(ZERO_MPS2.into(), ZERO_MPS2.into(), ZERO_MPS2.into()),
                time: ZERO_SECONDS.into(),
            },
        }
    }
}

impl<'a> Iterator for IterPointMassModel<'a> {
    type Item = (f64, f64, f64, f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        self.envelope.acceleration = self.drag_force() / self.model.mass() + self.model.g;
        self.envelope.position = self.envelope.position
            + self.envelope.velocity * self.model.time_step
            + self.envelope.acceleration * (self.model.time_step.powf(2.0) / 2.0);
        self.envelope.velocity =
            self.envelope.velocity + self.envelope.acceleration * self.model.time_step;
        self.envelope.time += self.model.time_step;
        Some((
            self.time(),
            self.velocity(),
            self.relative_distance(),
            self.relative_drop(),
            self.relative_windage(),
        ))
    }
}

impl<'a> DragSimulation for IterPointMassModel<'a> {
    fn rho(&self) -> f64 {
        let celsius = f64::from(self.model.temperature.to_celsius());
        let kelvin = f64::from(self.model.temperature.to_kelvin());
        let pa = f64::from(self.model.pressure.to_pascals());
        let pv = self.model.humidity
            * 611.21
            * E.powf((18.678 - (celsius / 234.5)) * (celsius / (257.14 + celsius)));
        let pd = pa - pv;
        ((pd * MOLAR_DRY) + (pv * MOLAR_VAPOR)) / (UNIVERSAL_GAS * kelvin)
    }
    fn mach(&self) -> f64 {
        let pa = f64::from(self.model.pressure.to_pascals());
        let c = (1.4 * (pa / self.rho())).sqrt();
        self.envelope.velocity.norm() / c
    }
    fn drag_force(&self) -> Vector3<f64> {
        let cd = self.model.drag_table.lerp(self.mach()) * self.model.i();
        let vv = self.envelope.velocity - self.model.wind_velocity;
        -(self.rho() * self.model.area() * vv * vv.norm() * cd) / 2.0
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

impl<'a> Output for IterPointMassModel<'a> {
    fn time(&self) -> f64 {
        f64::from(Time::Seconds(self.envelope.time).to_seconds())
    }
    fn velocity(&self) -> f64 {
        f64::from(Velocity::Mps(self.envelope.velocity.norm()).to_fps())
    }
    fn distance(&self) -> f64 {
        f64::from(Length::Meters(self.envelope.position.x).to_yards())
    }
    fn drop(&self) -> f64 {
        f64::from(Length::Meters(self.envelope.position.y).to_inches())
    }
    fn windage(&self) -> f64 {
        f64::from(Length::Meters(self.envelope.position.z).to_inches())
    }
    fn relative_velocity(&self) -> f64 {
        let angle = -self.model.initial_angle.to_radians();
        let axis = Vector3::z_axis();
        let rotation = Rotation3::from_axis_angle(&axis, angle);
        let velocity = rotation * self.envelope.velocity;
        f64::from(Velocity::Mps(velocity.norm()).to_fps())
    }
    fn relative_distance(&self) -> f64 {
        let angle = -self.model.initial_angle.to_radians();
        let axis = Vector3::z_axis();
        let rotation = Rotation3::from_axis_angle(&axis, angle);
        let position = rotation * self.envelope.position;
        f64::from(Length::Meters(position.x).to_yards())
    }
    fn relative_drop(&self) -> f64 {
        let angle = -self.model.initial_angle.to_radians();
        let axis = Vector3::z_axis();
        let rotation = Rotation3::from_axis_angle(&axis, angle);
        let height = Vector3::new(0.0, f64::from(self.model.scope_height.to_meters()), 0.0);
        let position = rotation * self.envelope.position - height;
        let drop = f64::from(Length::Meters(position.y));
        f64::from(Length::Meters(drop).to_inches())
    }
    fn relative_windage(&self) -> f64 {
        let angle = -self.model.initial_angle.to_radians();
        let axis = Vector3::z_axis();
        let rotation = Rotation3::from_axis_angle(&axis, angle);
        let position = rotation * self.envelope.position;
        f64::from(Length::Meters(position.z).to_inches())
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
                (Vector3::z_axis(), deg)
            }
            Wind(deg) => {
                // Rotation along y axis is yaw, wind left/right relative to x/z plane
                (Vector3::y_axis(), deg)
            }
        };
        let velocity_mps = vel.to_mps().into();
        let rotation = Rotation3::from_axis_angle(&axis, angle);
        let velocity = Vector3::new(velocity_mps, ZERO_MPS.into(), ZERO_MPS.into());
        rotation * velocity
    }
}
