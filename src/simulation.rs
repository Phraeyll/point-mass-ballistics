use na::{Rotation3, Vector3};

pub use dragtables::DragTableKind;

use self::constructors::*;
use conversions::*;
use dragtables::*;

use std::f64::consts::{E, PI};

// Constants used during drag calculation, and gravity during acceleration
const GRAVITY: f64 = -9.80665; // Local gravity in m/s
const UNIVERSAL_GAS: f64 = 8.314; // Universal gas constant (J/K*mol)
const MOLAR_DRY: f64 = 0.0289644; // Molar mass of dry air (kg/mol)
const MOLAR_VAPOR: f64 = 0.018016; // Molar mass of water vapor (kg/mol)

// All (most?) functions needed for drag calculation, and calculation itself
trait DragSimulation {
    fn area(&self) -> f64; // Area (meters)
    fn mass(&self) -> f64; // Mass (kgs)
    fn i(&self) -> f64; // Form Factor
    fn rho(&self) -> f64; // Density of air (kg/m^3)
    fn mach(&self) -> f64; // Velocity rel ative to speed of sound
    fn drag_force(&self) -> Vector3<f64>;
}

// Output accessor methods to get ballistic properties
pub trait Output {
    fn time(&self) -> f64;
    fn velocity(&self) -> f64;
    fn acceleration(&self) -> f64;
    fn distance(&self) -> f64;
    fn drop(&self) -> f64;
    fn windage(&self) -> f64;
    fn relative_position(&self) -> (f64, f64, f64);
}

// All variable required for running point mass model of trajectory simulation
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
    initial_angle: f64,              // Initial angle (radians), is also set in zero function
    first_zero: Vector3<f64>,        // First zero found after zero function
    pub time_step: f64,              // Timestep for simulation (s)
    pub initial_velocity: Velocity,  // Initial velocity (ft/s)
    pub scope_height: Length,        // Scope Height (inches)
    pub los_angle: f64,              // Line of Sight angle (degrees)
    pub drag_table: DragTable,       // Drag Function DragTable

    /*
    Other factors, not calculated yet
    pub ptmp: f64,                   // Powder Temperature (Modified Velocity?)
    pub lat:  f64,                   // Lattitude (Coriolis/Eotvos Effect)
    pub dir:  Direction,             // Direction Facing (Coriolis/Eotvos Effect)
    pub spin: f64,                   // Spin drift (Gyroscopic Drift)
    */
}

// Envelope of motion, changes over time through iter method
struct Envelope {
    // Envelope of motion
    time: f64,                  // Position in time (s)
    position: Vector3<f64>,     // Position (m)
    velocity: Vector3<f64>,     // Velocity (m/s)
    acceleration: Vector3<f64>, // Acceleration (m/s^2)
}

// Abstract iter struct for running simulation through iter method
pub struct IterPointMassModel<'a> {
    model: &'a PointMassModel,   // Reference to model used for calculations
    envelope: Envelope,          // Mutates through iteration, essentially the output as well
}

// Output struct for wrapping envelope of motion, provides accessor methods for convenience
// Mostly copied from IterPointMassModels envelope during iteration, some values from model
pub struct Ballistic {
    angle: f64,                 // Line of Sight Angle (radians)
    height: f64,                // Scope height (meters)
    time: f64,                  // Position in time (s)
    position: Vector3<f64>,     // Position (m)
    velocity: Vector3<f64>,     // Velocity (m/s)
    acceleration: Vector3<f64>, // Acceleration (m/s^2)
}

impl PointMassModel {
    // Create a new trajectory model, assuming all parameters are in traditional imperial units
    // All calculations are done using the SI system, mostly through trait methods on this struct
    // Wind velocity is exception - stored in m/s - need better consistency
    pub fn new(
        weight: f64,
        caliber: f64,
        bc: f64,
        initial_velocity: f64,
        scope_height: f64,
        los_angle: f64,
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

        Self {
            weight: weight_grains,
            caliber: diameter_inches,
            bc,

            wind_velocity: construct_velocity(wind_velocity_mph, Wind(wind_angle.to_radians())),
            temperature: temperature_f,
            pressure: pressure_inhg,
            humidity,
            g: Vector3::new(0.0, GRAVITY, 0.0),

            time_step: time_step_seconds.to_seconds().into(),
            initial_angle: 0.0,
            first_zero: Vector3::new(0.0, f64::from(scope_height_inches.to_meters()), 0.0),
            initial_velocity: initial_velocity_fps,
            scope_height: scope_height_inches,
            los_angle,
            drag_table: DragTable::new(drag_table),
        }
    }
    // Iterate over simulation, initializing with specified velocity
    pub fn iter<'a>(&'a self) -> IterPointMassModel {
        let angle = self.los_angle.to_radians() + self.initial_angle.to_radians();
        let initial_velocity_fps = self.initial_velocity;
        IterPointMassModel {
            model: self,
            envelope: Envelope {
                position: Vector3::new(0.0, 0.0, 0.0),
                velocity: construct_velocity(initial_velocity_fps, Projectile(angle)),
                acceleration: Vector3::new(0.0, 0.0, 0.0),
                time: 0.0,
            },
        }
    }
    // Find muzzle angle to achieve 0 drop at specified distance
    pub fn zero(&mut self, zero_distance: f64) {
        // Enums used to represent angling up or down, and functions to change directions
        enum Direction {
            Up,
            Down,
        }
        impl Direction {
            fn switch(&mut self) {
                *self = match *self {
                    Direction::Up => Direction::Down,
                    Direction::Down => Direction::Up,
                };
            }
        }
        fn switch_direction(direction: &mut Direction, angle: &mut f64) {
            direction.switch();
            *angle = -(*angle / 2.0);
        }

        // This angle will trace the longest possible trajectory for a projectile (45 degrees)
        const MAX_ANGLE: f64 = PI / 4.0;

        // Save line of sight, use 0 for finding 0
        // Will need to save other values later when accounting for cant and roll of scope
        // Wanted to reuse same iter method for simulation and zeroing
        // May need to find a better method for this, rather than mutating these here
        let old_los = self.los_angle;
        self.los_angle = 0.0;

        // Run the simulation to find the drop at a specified range.
        let zero = f64::from(self.scope_height.to_meters());
        let zero_distance_yards = Length::Yards(zero_distance);
        let zero_distance_meters = f64::from(zero_distance_yards.to_meters());

        // Start with maximum angle to allow for zeroing at longer distances
        let mut angle = MAX_ANGLE;
        // Start approach going up (must be the case due to gravity)
        let mut direction = Direction::Up;
        // Assume drop is negative from 0 degrees (must be the case due to gravity)
        let mut drop = -1.0;
        loop {
            self.initial_angle += angle;

            // Quit if algorithm goes above 45 degrees - will never be possible at this point
            if self.initial_angle > MAX_ANGLE {
                panic!("Can never 'zero' at this range")
            }

            // Find drop at distance
            for b in self.iter() {
                if b.position.x > zero_distance_meters {
                    drop = b.position.y;
                    break;
                }
            }
            // Quit once zero point is found, once drop is equal to zero
            if relative_eq!(drop, zero) {
                break;
            }

            // While going up, if drop is below zero, keep going up at same angle
            // While going down, if drop is above zero, keep going down at same angle
            match direction {
                Direction::Up => {
                    if drop > zero {
                        // If we crossed zero going up, change angle by 1/2 and direction
                        switch_direction(&mut direction, &mut angle);
                    }
                }
                Direction::Down => {
                    if drop < zero {
                        // If we crossed zero going down, change angle by 1/2 and change direction
                        switch_direction(&mut direction, &mut angle);
                    }
                }
            }
        }

        // Now find 'first' zero using the bore angle found for second zero
        // Algorithm above must find the second zero (projectile falling into zero) since
        // it starts with such a large angle.  The first zero is projectile rising to zero,
        // crossing line of sight while leaving the bore.  Will be used later for point blank range
        // calculations.
        let mut first_zero = Vector3::new(0.0, zero, 0.0);
        for b in self.iter() {
            if b.position.y > zero {
                first_zero = b.position;
                break;
            }
        }
        self.first_zero = first_zero;

        self.los_angle = old_los;
    }
    // Access first zero found
    pub fn first_zero(&self) -> (f64, f64, f64) {
        let (x, y, z) = (self.first_zero.x, self.first_zero.y, self.first_zero.z);
        let distance = f64::from(Length::Meters(x).to_yards());
        let drop = f64::from(Length::Meters(y).to_inches());
        let windage = f64::from(Length::Meters(z).to_inches());
        (distance, drop, windage)
    }
}

impl<'a> Iterator for IterPointMassModel<'a> {
    type Item = Ballistic;
    fn next(&mut self) -> Option<Self::Item> {
        // Acceleration from drag force and gravity (F = ma)
        self.envelope.acceleration = self.drag_force() / self.mass() + self.model.g;

        // Adjust position first, based on current position and acceleration
        self.envelope.position = self.envelope.position
            + self.envelope.velocity * self.model.time_step
            + self.envelope.acceleration * (self.model.time_step.powf(2.0) / 2.0);

        // Adjust velocity after new position is found, difference in position relative to time
        self.envelope.velocity =
            self.envelope.velocity + self.envelope.acceleration * self.model.time_step;

        // Increment position in time
        self.envelope.time += self.model.time_step;

        // Essentially a copy of current envelope of motion, plus los angle and scope height
        // for consumers
        Some(Ballistic {
            angle: self.model.los_angle.to_radians(),
            height: self.model.scope_height.to_meters().into(),
            time: self.envelope.time,
            position: self.envelope.position,
            velocity: self.envelope.velocity,
            acceleration: self.envelope.acceleration,
        })
    }
}

// Still not sure on this trait, not actually used anywhere
// Have ideas about "Modified Point Mass Model" that may be able to make use of traits/generics
impl<'a> DragSimulation for IterPointMassModel<'a> {
    // Area of projectil in kgs, used during drag force calculation
    fn area(&self) -> f64 {
        let radius = f64::from(self.model.caliber.to_meters()) / 2.0;
        PI * radius.powf(2.0)
    }
    // Mass of projectile in kgs, used during acceleration calculation in simulation iteration
    fn mass(&self) -> f64 {
        self.model.weight.to_kgs().into()
    }
    // Form factor of projectile, calculated fro Ballistic Coefficient and Sectional Density (sd)
    fn i(&self) -> f64 {
        let sd = f64::from(self.model.weight.to_lbs())
            / f64::from(self.model.caliber.to_inches()).powf(2.0);
        sd / self.model.bc
    }
    // Determine air density using Arden Buck equation given temperature and relative humidity
    fn rho(&self) -> f64 {
        let celsius = f64::from(self.model.temperature.to_celsius());
        let kelvin = f64::from(self.model.temperature.to_kelvin());

        // Total current pressure
        let pa = f64::from(self.model.pressure.to_pascals());

        // Pressure of water vapor
        let pv = self.model.humidity
            * 611.21
            * E.powf((18.678 - (celsius / 234.5)) * (celsius / (257.14 + celsius)));
        // Pressure of dry air
        let pd = pa - pv;

        ((pd * MOLAR_DRY) + (pv * MOLAR_VAPOR)) / (UNIVERSAL_GAS * kelvin)
    }
    // Determine velocity relative to speed of sound (c) with given atmospheric conditions
    fn mach(&self) -> f64 {
        let pa = f64::from(self.model.pressure.to_pascals());
        let c = (1.4 * (pa / self.rho())).sqrt();
        self.envelope.velocity.norm() / c
    }
    // Primary function - determines force of drag for given projectile, at given velocity
    // with given air density, using ballistic tables to modify coefficient of drag based on
    // standard reference projectiles (Eg., G1 or G7)
    // May be able to calculate wind at end of simulation, rather than iterate over timesteps
    // As there should be an analytical solution assuming the flight time is correctly determined
    // through this function.
    fn drag_force(&self) -> Vector3<f64> {
        let cd = self.model.drag_table.lerp(self.mach()) * self.i();
        let vv = self.envelope.velocity - self.model.wind_velocity;
        -(self.rho() * self.area() * vv * vv.norm() * cd) / 2.0
    }
}

// Accessor methods for getting common desired units of output
// Hard coded units for now - need to use better library for this eventually
impl Output for Ballistic {
    fn time(&self) -> f64 {
        f64::from(Time::Seconds(self.time).to_seconds())
    }
    fn velocity(&self) -> f64 {
        f64::from(Velocity::Mps(self.velocity.norm()).to_fps())
    }
    fn acceleration(&self) -> f64 {
        f64::from(Acceleration::Mps2(self.acceleration.norm()).to_fps2())
    }

    // Absolution positions, not relative to line of sight or scope height, imperial units
    fn distance(&self) -> f64 {
        f64::from(Length::Meters(self.position.x).to_yards())
    }
    fn drop(&self) -> f64 {
        f64::from(Length::Meters(self.position.y).to_inches())
    }
    fn windage(&self) -> f64 {
        f64::from(Length::Meters(self.position.z).to_inches())
    }

    // Supposed to show relative position of projectile against line of sight, which changes with
    // the angle of the shot.  Also offset by scope height.  Using rotation to rotate projectile
    // position to level ground, and substracting scope height to determine distance
    // I think this method is actually correct, but it needs more comparison against
    // other ballistic solvers, ideally other point mass models.  For certains projectiles,
    // this seems to be off 1-3 inches at 1000 yards vs jbm ballistics calculations
    fn relative_position(&self) -> (f64, f64, f64) {
        let angle = -self.angle;
        let axis = Vector3::z_axis();
        let rotation = Rotation3::from_axis_angle(&axis, angle);
        let height = Vector3::new(0.0, f64::from(self.height), 0.0);
        let position = rotation * self.position - height;
        (
            f64::from(Length::Meters(position.x).to_yards()),
            f64::from(Length::Meters(position.y).to_inches()),
            f64::from(Length::Meters(position.z).to_inches()),
        )
    }
}

// Module is probably overkill for this - just single method for building velocity based on angle
// Will need to extend to euler angles later on when roll/cant of scope is taken into account
mod constructors {
    pub use self::AngleKind::*;

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
        let velocity = Vector3::new(velocity_mps, 0.0, 0.0);
        rotation * velocity
    }
}
