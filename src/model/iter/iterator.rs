use nalgebra::Vector3;

use super::packet::*;
use super::physics::*;
use crate::model::core::{Atmosphere, Flags, Projectile, Scope, Shooter, Simulation, Wind};
use crate::util::*;

// Iterator over PointMassModel, steps through time and adjust position and velocity vectors
// Has reference to current simulation model for calculations
// Item lifetime also timed to this lifetime
#[derive(Debug)]
pub struct IterSimulation<'s> {
    simulation: &'s Simulation, // Reference to model used for calculations
    position: Vector3<Numeric>, // Position (m)
    velocity: Vector3<Numeric>, // Velocity (m/s)
    time: Numeric,              // Position in time (s)
}
impl InitIterator for Simulation {}
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

// Produce new 'packet', based on drag, coriolis acceleration, and gravity
// Contains time, position, and velocity of projectile, and reference to simulation used
impl<'s> Iterator for IterSimulation<'s> {
    type Item = Packet<'s, Simulation>;
    fn next(&mut self) -> Option<Self::Item> {
        // Previous values captured to be returned, so that time 0 can be accounted for
        let &mut Self {
            time,
            position,
            velocity,
            ..
        } = self;

        self.time += self.delta_time();
        self.position += self.delta_position();
        self.velocity += self.delta_velocity();

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

impl SimulationHandle for IterSimulation<'_> {
    type Simulation = Simulation;
    fn simulation(&self) -> &Self::Simulation {
        &self.simulation
    }
}

impl ParameterHandles for Simulation {
    fn flags(&self) -> &Flags {
        &self.flags
    }
    fn projectile(&self) -> &Projectile {
        &self.projectile
    }
    fn scope(&self) -> &Scope {
        &self.scope
    }
    fn shooter(&self) -> &Shooter {
        &self.shooter
    }
    fn atmosphere(&self) -> &Atmosphere {
        &self.atmosphere
    }
    fn wind(&self) -> &Wind {
        &self.wind
    }
    fn time_step(&self) -> Numeric {
        self.time_step
    }
}
impl Coriolis for IterSimulation<'_> {}
impl Drag for IterSimulation<'_> {}
impl Gravity for IterSimulation<'_> {}
impl GetMeasurement for IterSimulation<'_> {
    fn s_velocity(&self) -> Vector3<Numeric> {
        self.velocity
    }
    fn s_position(&self) -> Vector3<Numeric> {
        self.position
    }
    fn s_time(&self) -> Numeric {
        self.time
    }
}
