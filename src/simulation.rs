use na::{Rotation3, Vector3};
use of::OrderedFloat;

pub use dragtables::BallisticCoefficient;

use conversions::*;
use macros::FloatMap;
use util::*;

use std::iter::FromIterator;

// Z IS NOW DOWN/GRAVITY
// Constants used during drag calculation, and gravity during acceleration
const GRAVITY: Numeric = -9.806_65; // Local gravity in m/s
const UNIVERSAL_GAS: Numeric = 8.314; // Universal gas constant (J/K*mol)
const MOLAR_DRY: Numeric = 0.028_964_4; // Molar mass of dry air (kg/mol)
const MOLAR_VAPOR: Numeric = 0.018_016; // Molar mass of water vapor (kg/mol)
const ADIABATIC_INDEX_AIR: Numeric = 1.4; // Adiabatic index of air, mostly diatomic gas
const ANGULAR_VELOCITY_EARTH: Numeric = 0.000_072_921_159; // Angular velocity of earth, (radians)

pub struct Model {
    pub weight: WeightMass,            // Weight (grains)
    pub caliber: Length,               // Caliber (inches)
    pub bc: BallisticCoefficient,      // Ballistic Coefficient
    pub drag_table: FloatMap<Numeric>, // Drag Function DragTable
    pub time_step: Time,               // Timestep for simulation (s)
    pub muzzle_velocity: Velocity,     // Initial velocity (ft/s)
    pub scope_height: Length,          // Scope Height (inches)
}
impl Model {
    pub fn new(
        weight: Numeric,
        caliber: Numeric,
        bc: BallisticCoefficient,
        time_step: Numeric,
        muzzle_velocity: Numeric,
        scope_height: Numeric,
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
    // Radius of projectile cross section in meters
    fn radius(&self) -> Numeric {
        Numeric::from(self.caliber.to_meters()) / 2.0
    }
    // Area of projectile in meters, used during drag force calculation
    fn area(&self) -> Numeric {
        PI * self.radius().powf(2.0)
    }
    // Mass of projectile in kgs, used during acceleration calculation in simulation iteration
    fn mass(&self) -> Numeric {
        self.weight.to_kgs().into()
    }
    // Sectional density of projectile, defined terms of lbs and inches, yet dimensionless
    fn sd(&self) -> Numeric {
        Numeric::from(self.weight.to_lbs()) / Numeric::from(self.caliber.to_inches()).powf(2.0)
    }
    // Form factor of projectile, calculated fro Ballistic Coefficient and Sectional Density (sd)
    fn i(&self) -> Numeric {
        self.sd() / Numeric::from(self.bc)
    }
}

// Environmental Conditions and other varialbe for simulation
pub struct Conditions {
    pub temperature: Temperature,  // Temperature (F)
    pub pressure: Pressure,        // Pressure (InHg)
    pub humidity: Numeric,         // Humidity (0-1)
    pub gravity: Vector3<Numeric>, // Gravity (m/s^2)
    pub wind_velocity: Velocity,   // Wind Velocity (miles/hour)
    pub wind_yaw: Numeric,         // Wind Angle (degrees)
    pub shooter_pitch: Numeric,    // Line of Sight angle (degrees)
    pub azimuth: Numeric,          // Angle Facing (degrees) (Coriolis/Eotvos Effect)
    pub lattitude: Numeric,        // Lattitude (Coriolis/Eotvos Effect)

    /*
    Other factors, not calculated yet
    pub ptmp: Numeric,                   // Powder Temperature (Modified Velocity?)

    // Spin drift (Gyroscopic Drift)
    pub barrel_length: Numeric
    pub twist_ratio: Numeric

    */
}
impl Conditions {
    pub fn new(
        wind_velocity: Numeric,
        wind_yaw: Numeric,
        temperature: Numeric,
        pressure: Numeric,
        humidity: Numeric,
        shooter_pitch: Numeric,
        lattitude: Numeric,
        azimuth: Numeric,
    ) -> Self {
        Self {
            temperature: Temperature::F(temperature),
            pressure: Pressure::Inhg(pressure),
            humidity,
            gravity: Vector3::new(0.0, 0.0, GRAVITY),
            wind_velocity: Velocity::Mph(wind_velocity),
            wind_yaw: wind_yaw,
            shooter_pitch,
            lattitude,
            azimuth,
        }
    }
    // Rotated wind velocity vector according to angle along XY plane, relative
    // to shooter line of sight (X axis unit vector)
    fn wind_velocity(&self) -> Vector3<Numeric> {
        Rotation3::from_euler_angles(
            0.0,
            0.0,
            self.wind_yaw.to_radians() + self.azimuth.to_radians() - FRAC_PI_2,
        ) * Vector3::new(self.wind_velocity.to_mps().into(), 0.0, 0.0)
    }
    // Determine air density using Arden Buck equation given temperature and relative humidity
    fn rho(&self) -> Numeric {
        ((self.pd() * MOLAR_DRY) + (self.pv() * MOLAR_VAPOR)) / (UNIVERSAL_GAS * self.kelvin())
    }
    // Speed of sound
    fn c(&self) -> Numeric {
        (ADIABATIC_INDEX_AIR * (self.pa() / self.rho())).sqrt()
    }
    // Pressure of water vapor, Arden Buck equation
    fn pv(&self) -> Numeric {
        self.humidity
            * 611.21
            * ((18.678 - (self.celsius() / 234.5)) * (self.celsius() / (257.14 + self.celsius())))
                .exp()
    }
    // Pressure of dry air
    fn pd(&self) -> Numeric {
        self.pa() - self.pv()
    }
    // Total air pressure
    fn pa(&self) -> Numeric {
        Numeric::from(self.pressure.to_pascals())
    }
    // Temperature in celsius
    fn celsius(&self) -> Numeric {
        Numeric::from(self.temperature.to_celsius())
    }
    // Temperature in kelvin
    fn kelvin(&self) -> Numeric {
        Numeric::from(self.temperature.to_kelvin())
    }
}

// Distance => (drop, windage, velocity, energy, moa, time)
type TableVal = (Numeric, Numeric, Numeric, Numeric, Numeric, Numeric);
impl<T> FromIterator<(Numeric, T)> for FloatMap<T> {
    fn from_iter<I: IntoIterator<Item = (Numeric, T)>>(iter: I) -> Self {
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
        PointMassModel::new(
            &self.model,
            &self.solve_conditions,
            match self.zero_model().zero(zero_distance) {
                Ok(muzzle_pitch) => muzzle_pitch,
                Err(err) => panic!(err),
            },
        )
    }
    // Produce a drop table using specified range and step size
    pub fn drop_table<T>(
        &mut self,
        zero_distance: Numeric,
        step: Numeric,
        range: Numeric,
    ) -> FloatMap<T>
    where
        FloatMap<T>: FromIterator<(Numeric, TableVal)>,
    {
        let mut current_step: Numeric = 0.0;
        self.solution_model(Length::Yards(zero_distance))
            .iter()
            .take_do_while(|e| e.distance() < range)
            .filter_map(|e| {
                if e.distance() >= current_step {
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
    // pub fn first_zero(&self) -> Vector3<Numeric> {
    //     let zero = Numeric::from(self.model.scope_height.to_meters());
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
    muzzle_pitch: Numeric,
}
impl<'mc> PointMassModel<'mc> {
    // Create a new trajectory model, assuming all parameters are in traditional imperial units
    // All calculations are done using the SI system, mostly through trait methods on this struct
    // Wind velocity is exception - stored in m/s - need better consistency
    fn new(model: &'mc Model, conditions: &'mc Conditions, muzzle_pitch: Numeric) -> Self {
        Self {
            model,
            conditions,
            muzzle_pitch,
        }
    }
    // Find muzzle angle to achieve 0 drop at specified distance, relative to scope height
    fn zero(&mut self, zero_distance: Length) -> Result<Numeric, &'static str> {
        // This angle will trace the longest possible trajectory for a projectile (45 degrees)
        const MAX_ANGLE: Numeric = FRAC_PI_4;
        // Start with maximum angle to allow for zeroing at longer distances
        let mut angle = MAX_ANGLE;
        loop {
            let last_muzzle_pitch: Numeric = self.muzzle_pitch;
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
                .find(|e| e.relative_position().x > Numeric::from(zero_distance.to_meters()))
                .unwrap()
                .relative_position()
                .z;
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
    // Rotated velocity vector, accounts for muzzle/shooter pitch, and yaw (bearing)
    fn initial_velocity_vector(&self) -> Vector3<Numeric> {
        Rotation3::from_euler_angles(
            0.0,
            -(self.conditions.shooter_pitch.to_radians() + self.muzzle_pitch.to_radians()),
            self.conditions.azimuth.to_radians() - FRAC_PI_2,
        ) * Vector3::new(self.model.muzzle_velocity.to_mps().into(), 0.0, 0.0)
    }
    // Iterate over simulation, initializing with specified velocity
    fn iter(&self) -> IterPointMassModel {
        IterPointMassModel {
            simulation: self,
            position: Vector3::zeros(),
            velocity: self.initial_velocity_vector(),
            time: 0.0,
        }
    }
}

// Abstract iter struct for running simulation through iter method
// Essentially envelope of motion and ref to input variables
struct IterPointMassModel<'p> {
    simulation: &'p PointMassModel<'p>, // Reference to model used for calculations
    time: Numeric,                      // Position in time (s)
    position: Vector3<Numeric>,         // Position (m)
    velocity: Vector3<Numeric>,         // Velocity (m/s)
}
impl<'p> IterPointMassModel<'p> {
    // Initial coriolis equation, just for east/west drift right now
    // fn coriolis(&self) -> Numeric {
    //     -(2.0 / 3.0) // (8.0 / 3.0) for west
    //         * ANGULAR_VELOCITY_EARTH
    //         * self.position.y
    //         * self.simulation.conditions.lattitude.to_radians().sin()
    //         * (2.0 * self.position.y / GRAVITY).sqrt()
    // }
    // Determine velocity relative to speed of sound (c) with given atmospheric conditions
    fn coriolis_acceleration(&self) -> Vector3<Numeric> {
        // 2.0 * ANGULAR_VELOCITY_EARTH * Vector3::new(
        //     -vz * lattitude.sin() - vy * lattitude.cos() * azimuth.sin(),
        //     vz * lattitude.cos() * azimuth.cos() + vx * lattitude.cos() * azimuth.sin(),
        //     vx * lattitude.sin() - vy * lattitude.cos() * azimuth.cos(),
        // )
        let lattitude = self.simulation.conditions.lattitude.to_radians();
        let (ve, vn, vu) = (self.velocity.x, self.velocity.y, self.velocity.z);
        2.0 * ANGULAR_VELOCITY_EARTH * Vector3::new(
            vn * lattitude.sin() - vu * lattitude.cos(),
            -ve * lattitude.sin(),
            ve * lattitude.cos(),
        )
    }
    fn mach(&self) -> Numeric {
        self.velocity.norm() / self.simulation.conditions.c()
    }
    // Determine coefficient of drag used to determine drag force
    // Scaled by form factor of projectile
    fn cd(&self) -> Numeric {
        self.simulation.model.drag_table.lerp(self.mach()) * self.simulation.model.i()
    }
    // Velocity vector, after impact from wind (actually from drag, not "being blown")
    fn vv(&self) -> Vector3<Numeric> {
        self.velocity - self.simulation.conditions.wind_velocity()
    }
    // Primary function - determines force of drag for given projectile, at given mach speed,
    // with given air density, using ballistic tables to modify coefficient of drag based on
    // standard reference projectiles (Eg., G1 or G7)
    fn drag_force(&self) -> Vector3<Numeric> {
        -(self.simulation.conditions.rho()
            * self.simulation.model.area()
            * self.vv()
            * self.vv().norm()
            * self.cd())
            / 2.0
    }
}

// Iterate through simulation, outputting projectiles position in time and space, as well as current velocity
impl<'p> Iterator for IterPointMassModel<'p> {
    type Item = Envelope<'p>;
    fn next(&mut self) -> Option<Self::Item> {
        // Previous values, so we can capture time '0' in output
        let (time, position, velocity) = (self.time, self.position, self.velocity);
        // Unwrap time
        let time_step = Numeric::from(self.simulation.model.time_step.to_seconds());
        // Acceleration from drag force and gravity (F = ma)
        let acceleration = self.drag_force() / self.simulation.model.mass()
            + self.simulation.conditions.gravity
            + self.coriolis_acceleration();
        // Increment position in time
        self.time += time_step;
        // 'Second Equation of Motion'
        self.position += self.velocity * time_step + (acceleration * time_step.powf(2.0)) / 2.0;
        // 'First Equation of Motion'
        self.velocity += acceleration * time_step;
        // Currently envelope of motion (need better name) and ref to which simulation was used
        Some(Self::Item {
            simulation: &self.simulation,
            time,
            position,
            velocity,
        })
    }
}
impl<'p> IntoIterator for &'p PointMassModel<'p> {
    type Item = <IterPointMassModel<'p> as Iterator>::Item;
    type IntoIter = IterPointMassModel<'p>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Output struct for wrapping envelope of motion, provides accessor methods for convenience
// Mostly copied from IterPointMassModels envelope during iteration, some values from model
pub struct Envelope<'p> {
    simulation: &'p PointMassModel<'p>, //Simulation this came from, used for various calculations
    time: Numeric,                      // Position in time (s)
    position: Vector3<Numeric>,         // Position (m)
    velocity: Vector3<Numeric>,         // Velocity (m/s)
}
impl<'p> Envelope<'p> {
    // Supposed to show relative position of projectile against line of sight, which changes with
    // the angle of the shot.  Also offset by scope height.  Using rotation to rotate projectile
    // position to level ground, and substracts scope height to determine relative position
    // I think this method is actually correct, but it needs more comparison against
    // other ballistic solvers, ideally other point mass models.  For certains projectiles,
    // this seems to be off 1-3 inches at 1000 yards vs jbm ballistics calculations

    // Angle of line of sight (shooter_pitch)
    // Height of scope as vector, used to translate after rotation
    fn scope_height(&self) -> Vector3<Numeric> {
        Vector3::new(
            0.0,
            0.0,
            Numeric::from(self.simulation.model.scope_height.to_meters()),
        )
    }
    // Rotation matrix along z axis, sine this is the angle the shooter_pitch is along
    // Rotation point, then translate down to find position along oroginal origin
    // This should indicate relative position to line of sight along scopes axis
    fn relative_position(&self) -> Vector3<Numeric> {
        Rotation3::from_euler_angles(
            0.0,
            self.simulation.conditions.shooter_pitch.to_radians(),
            -(self.simulation.conditions.azimuth.to_radians() - FRAC_PI_2),
        ) * self.position
            - self.scope_height()
    }
}
// Output accessor methods to get ballistic properties
pub trait Output {
    fn time(&self) -> Numeric;
    fn velocity(&self) -> Numeric;
    fn energy(&self) -> Numeric;
    fn distance(&self) -> Numeric;
    fn drop(&self) -> Numeric;
    fn windage(&self) -> Numeric;
    fn moa(&self) -> Numeric;
}

// Accessor methods for getting common desired units of output
// Hard coded units for now - need to use better library for this eventually
impl<'p> Output for Envelope<'p> {
    fn time(&self) -> Numeric {
        Numeric::from(Time::Seconds(self.time).to_seconds())
    }
    fn velocity(&self) -> Numeric {
        Numeric::from(Velocity::Mps(self.velocity.norm()).to_fps())
    }
    fn energy(&self) -> Numeric {
        Numeric::from(
            Energy::Joules(self.simulation.model.mass() * self.velocity.norm().powf(2.0) / 2.0)
                .to_ftlbs(),
        )
    }
    // Positions relative to line of sight or scope height, imperial units
    fn distance(&self) -> Numeric {
        Numeric::from(Length::Meters(self.relative_position().x).to_yards())
    }
    fn drop(&self) -> Numeric {
        Numeric::from(Length::Meters(self.relative_position().z).to_inches())
    }
    fn windage(&self) -> Numeric {
        Numeric::from(Length::Meters(self.relative_position().y).to_inches())
    }
    fn moa(&self) -> Numeric {
        self.relative_position()
            .angle(&Vector3::x_axis())
            .to_degrees()
            * 60.0
    }
}
