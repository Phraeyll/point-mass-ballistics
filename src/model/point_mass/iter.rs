use nalgebra::Vector3;

use crate::util::*;

use std::ops::Sub;

// Iterator over PointMassModel, steps through time and adjust position and velocity vectors
// Has reference to current simulation model for calculations
// Output Item has this same reference
pub struct IterSimulation<'s> {
    simulation: &'s super::Simulation<'s>, // Reference to model used for calculations
    position: Vector3<Numeric>,            // Position (m)
    velocity: Vector3<Numeric>,            // Velocity (m/s)
    time: Numeric,                         // Position in time (s)
}

// Create an new iterator over Simulation
impl<'p> IntoIterator for &'p super::Simulation<'p> {
    type Item = <IterSimulation<'p> as Iterator>::Item;
    type IntoIter = IterSimulation<'p>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            simulation: self,
            position: Vector3::zeros(),
            velocity: self.muzzle_velocity_vector(),
            time: 0.0,
        }
    }
}

// Produce new 'packet', based on drag, coriolis acceleration, and gravity
// Contains time, position, and velocity of projectile, and reference to simulation used
impl<'s> Iterator for IterSimulation<'s> {
    type Item = Packet<'s>;
    fn next(&mut self) -> Option<Self::Item> {
        // Previous values captured to be returned, so that time 0 can be accounted for
        let &mut Self {
            time,
            position,
            velocity,
            ..
        } = self;

        // Unwrap time
        let time_step = self.simulation.time_step.to_seconds().to_num();
        // Acceleration from drag force and gravity (F = ma)
        // Keep drag acceleration for other uses
        let acceleration = self.drag_force() / self.simulation.projectile.mass()
            + self.simulation.conditions.other.gravity()
            + self.coriolis_acceleration();
        // Increment position in time
        self.time += time_step;
        // 'Second Equation of Motion'
        self.position += self.velocity * time_step + (acceleration * time_step.powf(2.0)) / 2.0;
        // 'First Equation of Motion'
        self.velocity += acceleration * time_step;

        let packet = Self::Item {
            simulation: &self.simulation,
            time,
            position,
            velocity,
        };
        // Only continue iteration for non terminal velocity, may change later
        if self.velocity.norm() != velocity.norm() {
            Some(packet)
        } else {
            println!(
                "Terminal velocity ({:.3} ft/s) reached at: {:.1} yards at angle: {:.2}",
                packet.velocity(),
                packet.distance(),
                packet.simulation.muzzle_pitch.to_degrees(),
            );
            None
        }
    }
}

impl IterSimulation<'_> {
    // Coriolis/Eotovos acceleration vector.  Accounts for Left/Right drift due to Earth's spin
    // This drift is always right (+z relative) in the northern hemisphere, regardless of initial bearing
    // This drive is always left (-z relative) in the southern hemisphere, regardless of initial bearing
    // Also accounts for elevation changes when launching projectils East/West, regardless of hemisphere
    // Bearing East results in higher elevation (+y absolute/relative)
    // Bearing West results in lower elevation (-y relative/absolute)
    fn coriolis_acceleration(&self) -> Vector3<Numeric> {
        -2.0 * self
            .simulation
            .conditions
            .other
            .omega()
            .cross(&self.velocity)
    }
    // Velocity relative to speed of sound (c), with given atmospheric conditions
    fn mach(&self) -> Numeric {
        self.velocity.norm() / self.simulation.conditions.atmosphere.speed_of_sound()
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
        -0.5 * self.simulation.conditions.atmosphere.rho()
            * self.simulation.projectile.area()
            * self.cd()
            * self.vv()
            * self.vv().norm()
    }
}

// Output struct which represents projectiles current position, and velocity
// Basically same values used internally during iteration
// Along with a ref to the simulation which was iterated over
pub struct Packet<'p> {
    simulation: &'p super::Simulation<'p>, //Simulation this came from, used for various calculations
    time: Numeric,                         // Position in time (s)
    position: Vector3<Numeric>,            // Position (m)
    velocity: Vector3<Numeric>,            // Velocity (m/s)
}
impl Packet<'_> {
    // During the simulation, the velocity of the projectile is rotated to allign with
    // the shooter's bearing (azimuth and line of sight)
    // This function returns the position rotated back to the initial frame of reference
    // This is used during zero'ing and is output in the drop table
    pub fn relative_position(&self) -> Vector3<Numeric> {
        self.position
            .yaw(-self.simulation.conditions.other.azimuth())
            .pitch(-self.simulation.conditions.other.line_of_sight())
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
impl Output for Packet<'_> {
    fn time(&self) -> Numeric {
        Time::Seconds(self.time).to_seconds().to_num()
    }
    fn velocity(&self) -> Numeric {
        Velocity::Mps(self.velocity.norm()).to_fps().to_num()
    }
    fn energy(&self) -> Numeric {
        Energy::Joules(self.simulation.projectile.mass() * self.velocity.norm().powf(2.0) / 2.0)
            .to_ftlbs()
            .to_num()
    }
    // Positions relative to line of sight (shooter_pitch)
    fn distance(&self) -> Numeric {
        Length::Meters(self.relative_position().x)
            .to_yards()
            .to_num()
    }
    fn drop(&self) -> Numeric {
        Length::Meters(self.relative_position().y)
            .to_inches()
            .to_num()
    }
    fn windage(&self) -> Numeric {
        Length::Meters(self.relative_position().z)
            .to_inches()
            .to_num()
    }
    fn moa(&self) -> Numeric {
        self.relative_position()
            .angle(&Vector3::x_axis())
            .to_degrees()
            * 60.0
    }
}
