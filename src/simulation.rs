use na::{Rotation3, Vector3};
use of::OrderedFloat;

pub use dragtables::BallisticCoefficient;

use conversions::*;
use macros::FloatMap;
use util::*;

use std::iter::FromIterator;

// Constants used during drag calculation, and gravity during acceleration
const GRAVITY: Numeric = -9.806_65; // Local gravity in m/s
const UNIVERSAL_GAS: Numeric = 8.314_459_8; // Universal gas constant (J/K*mol)
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
    fn scope_height(&self) -> Vector3<Numeric> {
        Vector3::new(0.0, 0.0, Numeric::from(self.scope_height.to_meters()))
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
    pub azimuth: Numeric,          // Bearing (0 North, 90 East) (degrees) (Coriolis/Eotvos Effect)
    pub lattitude: Numeric,        // Lattitude (Coriolis/Eotvos Effect)
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
    fn lattitude(&self) -> Numeric {
        self.lattitude.to_radians()
    }
    fn shooter_pitch(&self) -> Numeric {
        -self.shooter_pitch.to_radians()
    }
    fn wind_yaw(&self) -> Numeric {
        -self.wind_yaw.to_radians()
    }
    fn azimuth(&self) -> Numeric {
        -(self.azimuth.to_radians() - FRAC_PI_2)
    }
    // Velocity vector of wind, right now calculated only for horizontal winds.  Can add another
    // factor, wind_pitch, to consider vertical wind components
    fn wind_velocity(&self) -> Vector3<Numeric> {
        Rotation3::from_axis_angle(&Vector3::z_axis(), self.wind_yaw() + self.azimuth())
            * Vector3::new(self.wind_velocity.to_mps().into(), 0.0, 0.0)
    }
    // Density of air, using pressure, humidity, and temperature
    fn rho(&self) -> Numeric {
        ((self.pd() * MOLAR_DRY) + (self.pv() * MOLAR_VAPOR)) / (UNIVERSAL_GAS * self.kelvin())
    }
    // Speed of sound at given air density and pressure
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
    // Total air pressure in pascals
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
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    fn zero_model(&self) -> PointMassModel {
        PointMassModel::new(&self.model, &self.zero_conditions, 0.0)
    }
    // Create a simulation with muzzle pitch found in 'zeroin' simulation
    // Then solve for current conditions
    // Can be used for drop table, or eventually dialing in a specific distance
    fn solution_model(&self, zero_distance: Length) -> PointMassModel {
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
        &self,
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
            .take_do_while(|p| p.distance() < range)
            .filter_map(|p| {
                if p.distance() >= current_step {
                    current_step += step;
                    Some((
                        p.distance(), // Key
                        (
                            p.drop(),
                            p.windage(),
                            p.velocity(),
                            p.energy(),
                            p.moa(),
                            p.time(),
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
    //         if let Some(Projectile { position, .. }) = sim.next() {
    //             if position.y > zero {
    //                 break position;
    //             }
    //         }
    //     }
    // }
}

// Struct which runs the simulation - has iter method attached
struct PointMassModel<'mc> {
    model: &'mc Model,
    conditions: &'mc Conditions,
    muzzle_pitch: Numeric,
}
impl<'mc> PointMassModel<'mc> {
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
            // Find drop at distance, need way to break if we never zero_distance
            let drop = self
                .iter()
                .find(|p| p.relative_position().x > Numeric::from(zero_distance.to_meters()))
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
    fn muzzle_pitch(&self) -> Numeric {
        -self.muzzle_pitch.to_radians()
    }
    fn total_pitch(&self) -> Numeric {
        self.conditions.shooter_pitch() + self.muzzle_pitch()
    }
    // Rotated velocity vector, accounts for muzzle/shooter pitch, and yaw (bearing)
    // Start with velocity value along X unit vector
    fn initial_velocity_vector(&self) -> Vector3<Numeric> {
        Rotation3::from_axis_angle(&Vector3::z_axis(), self.conditions.azimuth())
            * Rotation3::from_axis_angle(&Vector3::y_axis(), self.total_pitch())
            * Vector3::new(self.model.muzzle_velocity.to_mps().into(), 0.0, 0.0)
    }
    // Create an iterator over the simulation model and conditions, starting with initial velocity
    fn iter(&self) -> IterPointMassModel {
        IterPointMassModel {
            simulation: self,
            position: Vector3::zeros(),
            velocity: self.initial_velocity_vector(),
            time: 0.0,
        }
    }
}

// Iterator over PointMassModel, steps through time and adjust position and velocity vectors
// Using reference to current simulation model/conditions
struct IterPointMassModel<'p> {
    simulation: &'p PointMassModel<'p>, // Reference to model used for calculations
    time: Numeric,                      // Position in time (s)
    position: Vector3<Numeric>,         // Position (m)
    velocity: Vector3<Numeric>,         // Velocity (m/s)
}
impl<'p> IterPointMassModel<'p> {
    // Angular velocity vector of earth, according with respect to current lattitude
    fn omega(&self) -> Vector3<Numeric> {
        ANGULAR_VELOCITY_EARTH * Vector3::new(
            0.0,
            self.simulation.conditions.lattitude().cos(),
            self.simulation.conditions.lattitude().sin(),
        )
    }
    // Coriolis/Eotovos acceleration vector.  Accounts for Left/Right drive dur to Earth's spin
    // This drift is always right (-y) in the northern hemisphere, regardless of initial bearing
    // This drive is always left (+y) in the southern hemisphere, regardless of initial bearing
    // Also accounts for elevation changes when launching projectils East/West, regardless of hemisphere
    // Bearing East results in higher elevation (+z), bearing West results in lower elevation (-z)
    fn coriolis_acceleration(&self) -> Vector3<Numeric> {
        -2.0 * self.omega().cross(&self.velocity)
    }
    // Velocity relative to speed of sound (c), with given atmospheric conditions
    fn mach(&self) -> Numeric {
        self.velocity.norm() / self.simulation.conditions.c()
    }
    // Coefficient of drag, scaled by the form factor of projectile referenced to a
    // particular standard projectile depending on drag table used
    fn cd(&self) -> Numeric {
        self.simulation.model.drag_table.lerp(self.mach()) * self.simulation.model.i()
    }
    // Velocity vector, after impact from wind (actually from drag, not "being blown")
    fn vv(&self) -> Vector3<Numeric> {
        self.velocity - self.simulation.conditions.wind_velocity()
    }
    // Force of drag for given projectile, at given mach speed, with given conditions
    // Drag force is proportional to square of velocity and area of projectile, scaled
    // by a coefficient at mach speeds (approximately)
    fn drag_force(&self) -> Vector3<Numeric> {
        -(self.simulation.conditions.rho()
            * self.simulation.model.area()
            * self.vv()
            * self.vv().norm()
            * self.cd())
            / 2.0
    }
}
impl<'p> Iterator for IterPointMassModel<'p> {
    type Item = Projectile<'p>;
    fn next(&mut self) -> Option<Self::Item> {
        // Previous values captured to be returned, so that time 0 can be accounted for
        // Would like a better method perhaps?
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

// Output struct which represents projectiles current position, and velocity
// Basically same values used internally during iteration
// Along with a ref to the simulation which was iterated over
pub struct Projectile<'p> {
    simulation: &'p PointMassModel<'p>, //Simulation this came from, used for various calculations
    time: Numeric,                      // Position in time (s)
    position: Vector3<Numeric>,         // Position (m)
    velocity: Vector3<Numeric>,         // Velocity (m/s)
}
impl<'p> Projectile<'p> {
    // During the simulation, the velocity of the projectile is rotate so it alligns with the shooter's bearing
    // and line of sight, listed here as azimuth and shooter_pitch - may rename later
    // This function rotates the projectiles point of position back to the initial coordinate system
    // where x_axis = East, y_axis = North, and z_axis = Elevation.  After rotation, the point is translated down
    // by the scope height, which should inidicate the points position relative to the line of sight.
    // This is used during zero'ing and output in the drop table
    fn relative_position(&self) -> Vector3<Numeric> {
        Rotation3::from_axis_angle(
            &Vector3::y_axis(),
            -self.simulation.conditions.shooter_pitch(),
        ) * Rotation3::from_axis_angle(&Vector3::z_axis(), -self.simulation.conditions.azimuth())
            * self.position
            - self.simulation.model.scope_height()
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

// Hard coded Imperial units for now - need to use better library for this eventually
impl<'p> Output for Projectile<'p> {
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
    // Positions relative to line of sight (shooter_pitch)
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
