use uom::si::acceleration::Acceleration;

use crate::{
    output::Packet,
    physics::DragFunction,
    simulation::Simulation,
    units::{acceleration, length, typenum::P2, velocity, ConstZero, Length, Time, Velocity},
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
            delta_position: MyVector3::new(Length::ZERO, Length::ZERO, Length::ZERO),
            delta_velocity: MyVector3::new(Velocity::ZERO, Velocity::ZERO, Velocity::ZERO),
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
    Self: Newtonian,
{
    type Item = Packet<'t, D>;

    fn next(&mut self) -> Option<Self::Item> {
        // Previous values captured to be returned, so that time 0 can be accounted for
        let &mut Self {
            time,
            delta_position,
            delta_velocity,
            ..
        } = self;
        let velocity = self.simulation.velocity() + self.delta_velocity;

        self.time += self.delta_time();
        self.delta_position += self.delta_position(velocity);
        self.delta_velocity += self.delta_velocity(velocity);

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
        if delta_position.get_x() != self.delta_position.get_x() {
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

pub trait Newtonian {
    fn acceleration(
        &self,
        _velocity: MyVector3<velocity::Dimension>,
    ) -> MyVector3<acceleration::Dimension> {
        MyVector3::new(Acceleration::ZERO, Acceleration::ZERO, Acceleration::ZERO)
    }

    fn delta_time(&self) -> Time;

    // 'Second Equation of Motion'
    fn delta_position(
        &self,
        velocity: MyVector3<velocity::Dimension>,
    ) -> MyVector3<length::Dimension> {
        velocity * self.delta_time()
            + (self.acceleration(velocity) * self.delta_time().powi(P2::new())) * 0.5
    }

    // 'First Equation of Motion'
    fn delta_velocity(
        &self,
        velocity: MyVector3<velocity::Dimension>,
    ) -> MyVector3<velocity::Dimension> {
        self.acceleration(velocity) * self.delta_time()
    }
}

impl<D> Newtonian for Iter<'_, D>
where
    D: DragFunction,
{
    fn acceleration(
        &self,
        velocity: MyVector3<velocity::Dimension>,
    ) -> MyVector3<acceleration::Dimension> {
        self.simulation.coriolis_acceleration(velocity)
            + self.simulation.drag_acceleration(velocity)
            + self.simulation.gravity_acceleration()
    }

    fn delta_time(&self) -> Time {
        self.simulation.time_step
    }
}
