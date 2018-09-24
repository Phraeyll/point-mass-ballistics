pub use crate::model::BallisticCoefficient;

use ordered_float::OrderedFloat;

use crate::{model, model::point_mass::iter::Output, util::*};

use std::iter::FromIterator;

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
    pub params: &'mzs model::point_mass::params::Unconditional, // Model variables, mostly projectile properties
    pub zero_conditions: &'mzs model::point_mass::params::Conditional, // Conditions used to find zero angle (muzzle_pitch)
    pub solve_conditions: &'mzs model::point_mass::params::Conditional, // Conditions used for dialing, drop tables, etc.
    pub time_step: Numeric,
}
impl<'mzs> Simulator<'mzs> {
    pub fn new(
        params: &'mzs model::point_mass::params::Unconditional,
        zero_conditions: &'mzs model::point_mass::params::Conditional,
        solve_conditions: &'mzs model::point_mass::params::Conditional,
        time_step: Numeric,
    ) -> Self {
        Self {
            params,
            zero_conditions,
            solve_conditions,
            time_step,
        }
    }
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    fn zero_simulation(&self) -> model::point_mass::Simulation {
        model::point_mass::Simulation::new(&self.params, &self.zero_conditions, 0.0, self.time_step)
    }
    // Create a simulation with muzzle pitch found in 'zeroin' simulation
    // Then solve for current conditions
    // Can be used for drop table, or eventually dialing in a specific distance
    fn solution_simulation(&self, zero_distance: Length) -> model::point_mass::Simulation {
        model::point_mass::Simulation::new(
            &self.params,
            &self.solve_conditions,
            match self.zero_simulation().zero(zero_distance) {
                Ok(muzzle_pitch) => muzzle_pitch,
                Err(err) => panic!(err),
            },
            self.time_step,
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
        self.solution_simulation(Length::Yards(zero_distance))
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

// Output accessor methods to get ballistic properties
