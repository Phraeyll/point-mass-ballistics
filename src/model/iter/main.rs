use nalgebra::Vector3;

use crate::model::core::Simulation;
use crate::util::*;
use super::packet::*;
// Ref iter
impl Simulation {
    pub fn iter(&self) -> IterSimulation {
        IterSimulation {
            simulation: self,
            position: self.absolute_projectile_position(),
            velocity: self.absolute_projectile_velocity(),
            time: 0.0,
        }
    }
}
// Create an new iterator over Simulation
impl<'s> IntoIterator for &'s Simulation {
    type Item = <IterSimulation<'s> as Iterator>::Item;
    type IntoIter = IterSimulation<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Iterator over PointMassModel, steps through time and adjust position and velocity vectors
// Has reference to current simulation model for calculations
// Output Item has this same reference
#[derive(Debug)]
pub struct IterSimulation<'s> {
    simulation: &'s Simulation, // Reference to model used for calculations
    position: Vector3<Numeric>, // Position (m)
    velocity: Vector3<Numeric>, // Velocity (m/s)
    time: Numeric,              // Position in time (s)
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

        let time_step = self.simulation.time_step;

        // Increment position in time
        self.time += time_step;
        // 'Second Equation of Motion'
        self.position +=
            self.velocity * time_step + (self.acceleration() * time_step.powf(2.0)) / 2.0;
        // 'First Equation of Motion'
        self.velocity += self.acceleration() * time_step;

        // Only continue iteration for changing 'forward' positions
        // Old check for norm may show up in false positives - norm could be same for 'valid' velocities
        // that are changing direction, but could still be traversion forward - norm loses information
        // It is only a magnitude.  It could be that the norm is the same for two different velocities
        // that are still moving forward, just at different angles
        //
        // This position check is still bad, however, as position may take a few ticks to change.
        // For practical purposes, this still may suffice.  I want to take this check out eventually, and
        // somehow allow caller to decide when to halt, ie, through filtering adaptors, although am not sure
        // how to check previous iteration values in standard iterator adaptors.
        if self.position.x != position.x {
            Some(Self::Item {
                simulation: &self.simulation,
                time,
                position,
                velocity,
            })
        } else {
            None
        }
    }
}
// Calculations used during iteration
impl IterSimulation<'_> {
    // Coriolis/Eotovos acceleration vector.  Accounts for Left/Right drift due to Earth's spin
    // This drift is always right (+z relative) in the northern hemisphere, regardless of initial bearing
    // This drive is always left (-z relative) in the southern hemisphere, regardless of initial bearing
    // Also accounts for elevation changes when launching projectils East/West, regardless of hemisphere
    // Bearing East results in higher elevation (+y absolute/relative)
    // Bearing West results in lower elevation (-y relative/absolute)
    fn coriolis_acceleration(&self) -> Vector3<Numeric> {
        if self.simulation.flags.use_coriolis {
            -2.0 * self
                .simulation
                .shooter
                .omega()
                .cross(&self.velocity)
        } else {
            Vector3::zeros()
        }
    }
    fn gravity_acceleration(&self) -> Vector3<Numeric> {
        if self.simulation.flags.use_gravity {
            self.simulation.shooter.gravity()
        } else {
            Vector3::zeros()
        }
    }
    fn drag_acceleration(&self) -> Vector3<Numeric> {
        if self.simulation.flags.use_drag {
            // Acceleration from drag force and gravity (F = ma)
            self.drag_force() / self.simulation.projectile.mass()
        } else {
            Vector3::zeros()
        }
    }
    fn acceleration(&self) -> Vector3<Numeric> {
        self.coriolis_acceleration() + self.drag_acceleration() + self.gravity_acceleration()
    }
    // Force of drag for given projectile, at given mach speed, with given conditions
    // Drag force is proportional to square of velocity and area of projectile, scaled
    // by a coefficient at mach speeds (approximately)
    fn drag_force(&self) -> Vector3<Numeric> {
        -0.5 * self.simulation.atmosphere.rho()
            * self.simulation.projectile.area()
            * self.cd()
            * self.simulation.projectile.i()
            * self.vv()
            * self.vv().norm()
    }
    // Coefficient of drag, as defined by a standard projectile depending on drag table used
    fn cd(&self) -> Numeric {
        self.simulation
            .projectile
            .bc
            .table()
            .lerp(self.mach())
            .expect("cd")
    }
    // Velocity relative to speed of sound (c), with given atmospheric conditions
    fn mach(&self) -> Numeric {
        self.velocity.norm() / self.simulation.atmosphere.speed_of_sound()
    }
    // Velocity vector, after impact from wind (actually from drag, not "being blown")
    // This is why the velocity from wind is subtracted, and vv is not used to find next velocity
    fn vv(&self) -> Vector3<Numeric> {
        self.velocity - self.simulation.absolute_wind_velocity()
    }
}
