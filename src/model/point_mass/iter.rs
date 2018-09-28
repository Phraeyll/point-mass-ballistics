use nalgebra::Vector3;

use crate::util::*;

use std::ops::Sub;

// Create an new iterator over Simulation
impl<'mc> super::Simulation<'mc> {
    // Create an iterator over the simulation model and conditions, starting with initial velocity
    pub fn iter(&self) -> IterSimulation {
        IterSimulation {
            simulation: self,
            position: Vector3::zeros(),
            velocity: self.muzzle_velocity_vector(),
            time: 0.0,
        }
    }
    // Rotated velocity vector, accounts for muzzle/shooter pitch, and yaw (bearing)
    // Start with velocity value along X unit vector
    fn muzzle_velocity_vector(&self) -> Vector3<Numeric> {
        self.projectile
            .velocity()
            .pitch(self.conditions.line_of_sight() + self.muzzle_pitch())
            .yaw(self.conditions.azimuth())
    }
    // Velocity vector of wind, right now calculated only for horizontal winds.  Can add another
    // factor, wind_pitch, to consider vertical wind components
    fn wind_velocity_vector(&self) -> Vector3<Numeric> {
        self.wind
            .velocity()
            .yaw(self.wind.yaw() + self.conditions.azimuth())
    }
}

// Struct which runs the simulation - has iter method attached
// Iterator over PointMassModel, steps through time and adjust position and velocity vectors
// Using reference to current simulation model/conditions
pub struct IterSimulation<'p> {
    simulation: &'p super::Simulation<'p>, // Reference to model used for calculations
    time: Numeric,                         // Position in time (s)
    position: Vector3<Numeric>,            // Position (m)
    velocity: Vector3<Numeric>,            // Velocity (m/s)
}

impl<'p> Iterator for IterSimulation<'p> {
    type Item = Projectile<'p>;
    fn next(&mut self) -> Option<Self::Item> {
        // Previous values captured to be returned, so that time 0 can be accounted for
        // Would like a better method perhaps?
        let (time, position, velocity) = (self.time, self.position, self.velocity);
        // Unwrap time
        let time_step = Numeric::from(self.simulation.time_step.to_seconds());
        // Acceleration from drag force and gravity (F = ma)
        let acceleration = self.drag_force() / self.simulation.projectile.mass()
            + self.simulation.conditions.gravity()
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
impl<'p> IterSimulation<'p> {
    // Coriolis/Eotovos acceleration vector.  Accounts for Left/Right drift due to Earth's spin
    // This drift is always right (+z relative) in the northern hemisphere, regardless of initial bearing
    // This drive is always left (-z relative) in the southern hemisphere, regardless of initial bearing
    // Also accounts for elevation changes when launching projectils East/West, regardless of hemisphere
    // Bearing East results in higher elevation (+y absolute/relative)
    // Bearing West results in lower elevation (-y relative/absolute)
    fn coriolis_acceleration(&self) -> Vector3<Numeric> {
        -2.0 * self.simulation.conditions.omega().cross(&self.velocity)
    }
    // Velocity relative to speed of sound (c), with given atmospheric conditions
    fn mach(&self) -> Numeric {
        self.velocity.norm() / self.simulation.atmosphere.speed_of_sound()
    }
    // Coefficient of drag, as defined by a standard projectile depending on drag table used
    fn cd(&self) -> Numeric {
        self.simulation.projectile.drag_table.lerp(self.mach()) * self.simulation.projectile.i()
    }
    // Velocity vector, after impact from wind (actually from drag, not "being blown")
    // This is why the velocity from wind is subtracted, and vv is not used to find next velocity
    fn vv(&self) -> Vector3<Numeric> {
        self.velocity - self.simulation.wind_velocity_vector()
    }
    // Force of drag for given projectile, at given mach speed, with given conditions
    // Drag force is proportional to square of velocity and area of projectile, scaled
    // by a coefficient at mach speeds (approximately)
    fn drag_force(&self) -> Vector3<Numeric> {
        -0.5 * self.simulation.atmosphere.rho()
            * self.simulation.projectile.area()
            * self.cd()
            * self.vv()
            * self.vv().norm()
    }
}
impl<'p> IntoIterator for &'p super::Simulation<'p> {
    type Item = <IterSimulation<'p> as Iterator>::Item;
    type IntoIter = IterSimulation<'p>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Output struct which represents projectiles current position, and velocity
// Basically same values used internally during iteration
// Along with a ref to the simulation which was iterated over
pub struct Projectile<'p> {
    simulation: &'p super::Simulation<'p>, //Simulation this came from, used for various calculations
    time: Numeric,                         // Position in time (s)
    position: Vector3<Numeric>,            // Position (m)
    velocity: Vector3<Numeric>,            // Velocity (m/s)
}
impl<'p> Projectile<'p> {
    // During the simulation, the velocity of the projectile is rotate so it alligns with the shooter's bearing
    // and line of sight, listed here as azimuth and shooter_pitch - may rename later
    // This function rotates the projectiles point of position back to the initial coordinate system
    // where x_axis = North/South, y_axis = Up/Down, and z_axis = East/West.  After rotation, the point is translated down
    // by the scope height, which should inidicate the points position relative to the line of sight.
    // This is used during zero'ing and output in the drop table
    pub fn relative_position(&self) -> Vector3<Numeric> {
        self.position
            .yaw(-self.simulation.conditions.azimuth())
            .pitch(-self.simulation.conditions.line_of_sight())
            .sub(self.simulation.scope.height())
    }
}

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
            Energy::Joules(
                self.simulation.projectile.mass() * self.velocity.norm().powf(2.0) / 2.0,
            )
            .to_ftlbs(),
        )
    }
    // Positions relative to line of sight (shooter_pitch)
    fn distance(&self) -> Numeric {
        Numeric::from(Length::Meters(self.relative_position().x).to_yards())
    }
    fn drop(&self) -> Numeric {
        Numeric::from(Length::Meters(self.relative_position().y).to_inches())
    }
    fn windage(&self) -> Numeric {
        Numeric::from(Length::Meters(self.relative_position().z).to_inches())
    }
    fn moa(&self) -> Numeric {
        self.relative_position()
            .angle(&Vector3::x_axis())
            .to_degrees()
            * 60.0
    }
}
