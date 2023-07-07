use crate::{
    output::Packet,
    physics::DragFunction,
    simulation::Simulation,
    units::{length, typenum::P2, velocity, ConstZero, Time},
    vectors::MyVector3,
};

use std::iter::FusedIterator;

// Iterator over PointMassModel, steps through time and adjust position and velocity vectors
// Has reference to current simulation model for calculations
// Item lifetime also timed to this lifetime
#[derive(Debug)]
pub struct Iter<'t, D> {
    simulation: &'t Simulation<D>, // Reference to model used for calculations
    delta_position: MyVector3<length::Dimension>, // Position (m)
    delta_velocity: MyVector3<velocity::Dimension>, // Velocity (m/s)
    time: Time,                    // Position in time (s)
}

impl<D> Simulation<D> {
    pub fn iter(&self) -> Iter<'_, D> {
        Iter {
            simulation: self,
            delta_position: MyVector3::ZERO,
            delta_velocity: MyVector3::ZERO,
            time: Time::ZERO,
        }
    }
}

// Create an new iterator over Simulation
impl<'t, D> IntoIterator for &'t Simulation<D>
where
    D: DragFunction,
{
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = Iter<'t, D>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Produce new 'packet', based on drag, coriolis acceleration, and gravity
// Contains time, position, and velocity of projectile, and reference to simulation used
impl<'t, D> Iterator for Iter<'t, D>
where
    D: DragFunction,
{
    type Item = Packet<'t, D>;

    fn next(&mut self) -> Option<Self::Item> {
        // Previous values captured to be returned, so that time 0 can be accounted for
        let &mut Self {
            ref simulation,
            time,
            delta_position,
            delta_velocity,
            ..
        } = self;
        let velocity = simulation.velocity() + delta_velocity;

        let dt = simulation.time_step;
        let dt_sq = dt.powi(P2::new());
        let a = simulation.acceleration(velocity);

        // Second Equation of Motion
        let dp = velocity * dt + a * dt_sq * 0.5;

        // First Equation of Motion
        let dv = a * dt;

        self.time += dt;
        self.delta_position += dp;
        self.delta_velocity += dv;

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
        if delta_velocity.get_x() != self.delta_velocity.get_x() {
            Some(Self::Item {
                simulation: self.simulation,
                time,
                delta_position,
                delta_velocity,
            })
        } else {
            None
        }
    }
}

impl<'t, D> FusedIterator for Iter<'t, D> where D: DragFunction {}
