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
    terminal: bool, // Records if last velocity was same as current (terminal velocity)
}

impl<D> Simulation<D> {
    pub fn iter(&self) -> Iter<'_, D> {
        Iter {
            simulation: self,
            delta_position: MyVector3::ZERO,
            delta_velocity: MyVector3::ZERO,
            time: Time::ZERO,
            terminal: false,
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

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        // Previous values captured to be returned, so that time 0 can be accounted for
        let &mut Self {
            simulation,
            delta_position,
            delta_velocity,
            time,
            ..
        } = self;
        let v = simulation.velocity() + delta_velocity;

        let dt = simulation.time_step;
        let dt_sq = dt.powi(P2::new());
        let a = simulation.acceleration(v);

        // Second Equation of Motion
        let dp = v * dt + a * dt_sq * 0.5;

        // First Equation of Motion
        let dv = a * dt;

        self.time += dt;
        self.delta_position += dp;
        self.delta_velocity += dv;

        // Check is projectile is moving "forward" - stop iteration if not
        // Close/Equal to terminal velocity
        if !self.terminal {
            if delta_position.get_x() == self.delta_position.get_x() {
                self.terminal = true;
            }
            Some(Self::Item {
                simulation,
                delta_position,
                delta_velocity,
                time,
            })
        } else {
            None
        }
    }
}

impl<'t, D> FusedIterator for Iter<'t, D> where D: DragFunction {}
