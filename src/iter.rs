use crate::{
    output::Packet,
    physics::DragFunction,
    simulation::Simulation,
    units::{ConstZero, Time, length, typenum::P2, velocity},
    vectors::MyVector3,
};

use std::iter::FusedIterator;

#[derive(Debug)]
pub struct Iter<'a, D> {
    simulation: &'a Simulation<D>,
    position: MyVector3<length::Dimension>,
    velocity: MyVector3<velocity::Dimension>,
    time: Time,
}

impl<D> Simulation<D> {
    pub fn iter(&self) -> Iter<'_, D> {
        Iter {
            simulation: self,
            position: MyVector3::ZERO,
            velocity: MyVector3::ZERO,
            time: Time::ZERO,
        }
    }
}

impl<'a, D> IntoIterator for &'a Simulation<D>
where
    D: DragFunction,
{
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = Iter<'a, D>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, D> Iterator for Iter<'a, D>
where
    D: DragFunction,
{
    type Item = Packet<'a, D>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        // Previous values captured to be returned, so that time 0 can be accounted for
        let &mut Self {
            simulation,
            position,
            velocity,
            time,
        } = self;

        let v = simulation.velocity() + velocity;
        let dt = simulation.time_step;
        let dt_sq = dt.powi(P2::new());
        let a = simulation.acceleration(v);

        // Second Equation of Motion
        let dp = v * dt + a * dt_sq * 0.5;

        // First Equation of Motion
        let dv = a * dt;

        self.time += dt;
        self.position += dp;
        self.velocity += dv;

        Some(Self::Item {
            simulation,
            time,
            position,
            velocity,
            acceleration: a,
        })
    }
}

impl<'a, D> FusedIterator for Iter<'a, D> where D: DragFunction {}
