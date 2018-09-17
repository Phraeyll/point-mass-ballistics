use na::{Rotation3, Vector3};
use of::OrderedFloat;

pub use dragtables::BallisticCoefficient;

use self::constructors::*;
use conversions::*;
use macros::FloatMap;
use util::*;

use std::f64::consts::PI;
use std::iter::FromIterator;

// Constants used during drag calculation, and gravity during acceleration
const GRAVITY: f64 = -9.806_65; // Local gravity in m/s
const UNIVERSAL_GAS: f64 = 8.314; // Universal gas constant (J/K*mol)
const MOLAR_DRY: f64 = 0.028_964_4; // Molar mass of dry air (kg/mol)
const MOLAR_VAPOR: f64 = 0.018_016; // Molar mass of water vapor (kg/mol)
const ADIABATIC_INDEX_AIR: f64 = 1.4; // Adiabatic index of air, mostly diatomic gas

pub struct Model {
    pub weight: WeightMass,        // Weight (grains)
    pub caliber: Length,           // Caliber (inches)
    pub bc: BallisticCoefficient,  // Ballistic Coefficient
    pub drag_table: FloatMap<f64>, // Drag Function DragTable
    pub time_step: Time,           // Timestep for simulation (s)
    pub muzzle_velocity: Velocity, // Initial velocity (ft/s)
    pub scope_height: Length,      // Scope Height (inches)
}
impl Model {
    pub fn new(
        weight: f64,
        caliber: f64,
        bc: BallisticCoefficient,
        time_step: f64,
        muzzle_velocity: f64,
        scope_height: f64,
    ) -> Self {
        Self {
            weight: WeightMass::Grains(weight),
            caliber: Length::Inches(caliber),
            bc,
            drag_table: bc.table(),
            time_step: Time::Seconds(time_step),
            muzzle_velocity: Velocity::Fps(muzzle_velocity),
            scope_height: Length::Inches(scope_height),
        }
    }
    // Area of projectile in meters, used during drag force calculation
    fn area(&self) -> f64 {
        let radius = f64::from(self.caliber.to_meters()) / 2.0;
        PI * radius.powf(2.0)
    }
    // Mass of projectile in kgs, used during acceleration calculation in simulation iteration
    fn mass(&self) -> f64 {
        self.weight.to_kgs().into()
    }
    fn sd(&self) -> f64 {
        f64::from(self.weight.to_lbs()) / f64::from(self.caliber.to_inches()).powf(2.0)
    }
    // Form factor of projectile, calculated fro Ballistic Coefficient and Sectional Density (sd)
    fn i(&self) -> f64 {
        self.sd() / f64::from(self.bc)
    }
}

// Environmental Conditions and other varialbe for simulation
pub struct Conditions {
    pub temperature: Temperature, // Temperature (F)
    pub pressure: Pressure,       // Pressure (InHg)
    pub humidity: f64,            // Humidity (0-1)
    pub gravity: Vector3<f64>,    // Gravity (m/s^2)
    pub wind_velocity: Velocity,  // Wind Velocity (miles/hour)
    pub wind_yaw: f64,            // Wind Angle (degrees)
    pub shooter_pitch: f64,       // Line of Sight angle (degrees)

    /*
    Other factors, not calculated yet
    pub ptmp: f64,                   // Powder Temperature (Modified Velocity?)
    pub lat:  f64,                   // Lattitude (Coriolis/Eotvos Effect)
    pub dir:  Direction,             // Direction Facing (Coriolis/Eotvos Effect)
    pub spin: f64,                   // Spin drift (Gyroscopic Drift)
    */
}
impl Conditions {
    pub fn new(
        wind_velocity: f64,
        wind_yaw: f64,
        temperature: f64,
        pressure: f64,
        humidity: f64,
        shooter_pitch: f64,
    ) -> Self {
        Self {
            temperature: Temperature::F(temperature),
            pressure: Pressure::Inhg(pressure),
            humidity,
            gravity: Vector3::new(0.0, GRAVITY, 0.0),
            wind_velocity: Velocity::Mph(wind_velocity),
            wind_yaw: wind_yaw,
            shooter_pitch,
        }
    }
    fn wind_velocity(&self) -> Vector3<f64> {
        velocity_vector(self.wind_velocity, &AngleKind::Wind(self.wind_yaw))
    }
    // Determine air density using Arden Buck equation given temperature and relative humidity
    fn rho(&self) -> f64 {
        ((self.pd() * MOLAR_DRY) + (self.pv() * MOLAR_VAPOR)) / (UNIVERSAL_GAS * self.kelvin())
    }
    // Speed of sound
    fn c(&self) -> f64 {
        (ADIABATIC_INDEX_AIR * (self.pa() / self.rho())).sqrt()
    }
    // Pressure of water vapor, Arden Buck equation
    fn pv(&self) -> f64 {
        self.humidity
            * 611.21
            * ((18.678 - (self.celsius() / 234.5)) * (self.celsius() / (257.14 + self.celsius())))
                .exp()
    }
    // Pressure of dry air
    fn pd(&self) -> f64 {
        self.pa() - self.pv()
    }
    // Total air pressure
    fn pa(&self) -> f64 {
        f64::from(self.pressure.to_pascals())
    }
    // Temperature in celsius
    fn celsius(&self) -> f64 {
        f64::from(self.temperature.to_celsius())
    }
    // Temperature in kelvin
    fn kelvin(&self) -> f64 {
        f64::from(self.temperature.to_kelvin())
    }
}

// Distance => (drop, windage, velocity, energy, moa, time)
type TableVal = (f64, f64, f64, f64, f64, f64);
impl<T> FromIterator<(f64, T)> for FloatMap<T> {
    fn from_iter<I: IntoIterator<Item = (f64, T)>>(iter: I) -> Self {
        let mut drop_table = FloatMap::<T>::default();
        for i in iter {
            drop_table.0.insert(OrderedFloat(i.0), i.1);
        }
        drop_table
    }
}

pub struct Simulator<'mzs> {
    pub model: &'mzs Model, // Model variables, mostly projectile properties
    pub zero_conditions: &'mzs Conditions, // Conditions used to find zero angle (muzzle_pitch)
    pub solve_conditions: &'mzs Conditions, // Conditions used for dialing, drop tables, etc.
}
impl<'mzs> Simulator<'mzs> {
    pub fn new(
        model: &'mzs Model,
        zero_conditions: &'mzs Conditions,
        solve_conditions: &'mzs Conditions,
    ) -> Self {
        Self {
            model,
            zero_conditions,
            solve_conditions,
        }
    }
    // Create simulation with conditions used to find zero angle
    // Ensure current muzzle pitch is 0 before running simulation
    fn zero_model(&mut self) -> PointMassModel {
        PointMassModel::new(&self.model, &self.zero_conditions, 0.0)
    }
    // Find zero angle, then solve for current conditions
    fn solution_model(&mut self, zero_distance: Length) -> PointMassModel {
        let muzzle_pitch = match self.zero_model().zero(zero_distance) {
            Ok(muzzle_pitch) => muzzle_pitch,
            Err(err) => panic!(err),
        };
        PointMassModel::new(&self.model, &self.solve_conditions, muzzle_pitch)
    }
    // Produce a drop table using specified range and step size
    pub fn drop_table<T>(&mut self, zero_distance: f64, step: f64, range: f64) -> FloatMap<T>
    where
        FloatMap<T>: FromIterator<(f64, TableVal)>,
    {
        let mut current_step: f64 = 0.0;
        self.solution_model(Length::Yards(zero_distance))
            .iter()
            .take_do_while(|e| e.distance() < range)
            .filter_map(|e| {
                if e.distance() > current_step {
                    current_step += step;
                    Some((
                        e.distance(), // Key
                        (
                            e.drop(),
                            e.windage(),
                            e.velocity(),
                            e.energy(),
                            e.moa(),
                            e.time(),
                        ), // Value
                    ))
                } else {
                    None
                }
            }).collect::<FloatMap<T>>()
    }
    // // Need way to produce or find first zero for PBR calculations
    // pub fn first_zero(&self) -> Vector3<f64> {
    //     let zero = f64::from(self.model.scope_height.to_meters());
    //     let mut sim = PointMassModel::new(&mut self.model, &self.zero_conditions).iter();
    //     loop {
    //         if let Some(Envelope { position, .. }) = sim.next() {
    //             if position.y > zero {
    //                 break position;
    //             }
    //         }
    //     }
    // }
}

// All variable required for running point mass model of trajectory simulation
struct PointMassModel<'mc> {
    model: &'mc Model,           // Other variables used in point mass model
    conditions: &'mc Conditions, // Conditions that vary depending on simulation type
    muzzle_pitch: f64,
}
impl<'mc> PointMassModel<'mc> {
    // Create a new trajectory model, assuming all parameters are in traditional imperial units
    // All calculations are done using the SI system, mostly through trait methods on this struct
    // Wind velocity is exception - stored in m/s - need better consistency
    fn new(model: &'mc Model, conditions: &'mc Conditions, muzzle_pitch: f64) -> Self {
        Self {
            model,
            conditions,
            muzzle_pitch,
        }
    }
    // Find muzzle angle to achieve 0 drop at specified distance, relative to scope height
    fn zero(&mut self, zero_distance: Length) -> Result<f64, &'static str> {
        // This angle will trace the longest possible trajectory for a projectile (45 degrees)
        const MAX_ANGLE: f64 = PI / 4.0;

        // Run the simulation to find the drop at a specified range.
        let zero_distance_meters = f64::from(zero_distance.to_meters());

        // Start with maximum angle to allow for zeroing at longer distances
        let mut angle = MAX_ANGLE;

        loop {
            let last_muzzle_pitch: f64 = self.muzzle_pitch;
            self.muzzle_pitch += angle;
            if self.muzzle_pitch > MAX_ANGLE {
                break Err("Can never 'zero' at this range");
            }
            if self.muzzle_pitch == last_muzzle_pitch {
                break Err("Issue with floating points, angle not changing during 'zero'");
            }
            // Find drop at distance, need way to break if we never reach position.x
            let drop = self
                .iter()
                .find(|e| e.relative_position().x > zero_distance_meters)
                .unwrap()
                .relative_position()
                .y;
            // Quit once zero point is found, once drop is equal to zero
            if relative_eq!(drop, 0.0) {
                break Ok(self.muzzle_pitch);
            }
            // If in the following states (xor), change direction by flipping angle sign
            // true, false || false, true
            // up,   above || down,  below
            if angle.is_sign_positive() ^ drop.is_sign_negative() {
                angle *= -1.0;
            }
            // Reduce angle before next iteration, trying to converge on zero point
            angle /= 2.0;
        }
    }
    // Iterate over simulation, initializing with specified velocity
    fn iter(&self) -> IterPointMassModel {
        IterPointMassModel {
            simulation: self,
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: velocity_vector(
                self.model.muzzle_velocity,
                &AngleKind::Projectile(self.muzzle_pitch + self.conditions.shooter_pitch),
            ),
            time: 0.0,
        }
    }
}

impl<'p> IntoIterator for &'p PointMassModel<'p> {
    type Item = <IterPointMassModel<'p> as Iterator>::Item;
    type IntoIter = IterPointMassModel<'p>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Abstract iter struct for running simulation through iter method
// Essentially envelope of motion and ref to input variables
struct IterPointMassModel<'p> {
    simulation: &'p PointMassModel<'p>, // Reference to model used for calculations
    time: f64,                          // Position in time (s)
    position: Vector3<f64>,             // Position (m)
    velocity: Vector3<f64>,             // Velocity (m/s)
}
impl<'p> IterPointMassModel<'p> {
    // Determine velocity relative to speed of sound (c) with given atmospheric conditions
    fn mach(&self) -> f64 {
        self.velocity.norm() / self.simulation.conditions.c()
    }
    // Primary function - determines force of drag for given projectile, at given mach speed,
    // with given air density, using ballistic tables to modify coefficient of drag based on
    // standard reference projectiles (Eg., G1 or G7)
    fn drag_force(&self) -> Vector3<f64> {
        let cd = self.simulation.model.drag_table.lerp(self.mach()) * self.simulation.model.i();
        let vv = self.velocity - self.simulation.conditions.wind_velocity();
        -(self.simulation.conditions.rho() * self.simulation.model.area() * vv * vv.norm() * cd)
            / 2.0
    }
}

impl<'p> Iterator for IterPointMassModel<'p> {
    type Item = Envelope<'p>;
    fn next(&mut self) -> Option<Self::Item> {
        let time_step = f64::from(self.simulation.model.time_step.to_seconds());
        // Acceleration from drag force and gravity (F = ma)
        let acceleration =
            self.drag_force() / self.simulation.model.mass() + self.simulation.conditions.gravity;

        // Adjust position first, to keep previous velocity for First Equation
        // 'Second Equation of Motion'
        self.position += self.velocity * time_step + (acceleration * time_step.powf(2.0)) / 2.0;

        // Adjust velocity next, based on change in acceleration
        // 'First Equation of Motion'
        self.velocity += acceleration * time_step;

        // Increment position in time
        self.time += time_step;

        // Essentially a copy of current envelope of motion, plus los angle and scope height
        // for consumers
        Some(Self::Item {
            simulation: &self.simulation,
            time: self.time,
            position: self.position,
            velocity: self.velocity,
        })
    }
}

// Output struct for wrapping envelope of motion, provides accessor methods for convenience
// Mostly copied from IterPointMassModels envelope during iteration, some values from model
pub struct Envelope<'p> {
    simulation: &'p PointMassModel<'p>, //Simulation this came from, used for various calculations
    time: f64,                          // Position in time (s)
    position: Vector3<f64>,             // Position (m)
    velocity: Vector3<f64>,             // Velocity (m/s)
}
impl<'p> Envelope<'p> {
    // Supposed to show relative position of projectile against line of sight, which changes with
    // the angle of the shot.  Also offset by scope height.  Using rotation to rotate projectile
    // position to level ground, and substracting scope height to determine distance
    // I think this method is actually correct, but it needs more comparison against
    // other ballistic solvers, ideally other point mass models.  For certains projectiles,
    // this seems to be off 1-3 inches at 1000 yards vs jbm ballistics calculations
    fn relative_position(&self) -> Vector3<f64> {
        let angle = -self.simulation.conditions.shooter_pitch.to_radians();
        let height = f64::from(self.simulation.model.scope_height.to_meters());
        let axis = Vector3::z_axis();
        let rotation = Rotation3::from_axis_angle(&axis, angle);
        let height = Vector3::new(0.0, height, 0.0);
        rotation * self.position - height
    }
}
// Output accessor methods to get ballistic properties
pub trait Output {
    fn time(&self) -> f64;
    fn velocity(&self) -> f64;
    fn energy(&self) -> f64;
    fn distance(&self) -> f64;
    fn drop(&self) -> f64;
    fn windage(&self) -> f64;
    fn moa(&self) -> f64;
}

// Accessor methods for getting common desired units of output
// Hard coded units for now - need to use better library for this eventually
impl<'p> Output for Envelope<'p> {
    fn time(&self) -> f64 {
        f64::from(Time::Seconds(self.time).to_seconds())
    }
    fn velocity(&self) -> f64 {
        f64::from(Velocity::Mps(self.velocity.norm()).to_fps())
    }
    fn energy(&self) -> f64 {
        f64::from(
            Energy::Joules(self.simulation.model.mass() * self.velocity.norm().powf(2.0) / 2.0)
                .to_ftlbs(),
        )
    }
    // Positions relative to line of sight or scope height, imperial units
    fn distance(&self) -> f64 {
        f64::from(Length::Meters(self.relative_position().x).to_yards())
    }
    fn drop(&self) -> f64 {
        f64::from(Length::Meters(self.relative_position().y).to_inches())
    }
    fn windage(&self) -> f64 {
        f64::from(Length::Meters(self.relative_position().z).to_inches())
    }
    fn moa(&self) -> f64 {
        let r = self.relative_position().y;
        let h = self.relative_position().x;
        let s = (h.powf(2.0) + r.powf(2.0)).sqrt();
        (r / s).asin().to_degrees() * 60.0
    }
}

// Module is probably overkill for this - just single method for building velocity based on angle
// Will need to extend to euler angles later on when roll/cant of scope is taken into account
mod constructors {
    use conversions::*;
    use na::{Rotation3, Vector3};

    pub enum AngleKind {
        Projectile(f64),
        Wind(f64),
    }

    pub fn velocity_vector(vel: Velocity, vk: &AngleKind) -> Vector3<f64> {
        let (axis, angle) = match *vk {
            AngleKind::Projectile(rad) => {
                // Rotation along z axis is pitch, projectile up/down relative to x/y plane
                (Vector3::z_axis(), rad)
            }
            AngleKind::Wind(rad) => {
                // Rotation along y axis is yaw, wind left/right relative to x/z plane
                (Vector3::y_axis(), rad)
            }
        };
        let velocity_mps = vel.to_mps().into();
        let rotation = Rotation3::from_axis_angle(&axis, angle.to_radians());
        let velocity = Vector3::new(velocity_mps, 0.0, 0.0);
        rotation * velocity
    }
}
