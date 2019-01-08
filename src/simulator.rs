pub use crate::model::BallisticCoefficient;

use crate::{
    model,
    model::point_mass::{iter::Output, *},
    util::*,
};

// Distance => (drop, windage, velocity, energy, moa, time)
type TableVal = (Numeric, Numeric, Numeric, Numeric, Numeric, Numeric);

pub struct Simulator<'p> {
    pub projectile: &'p Projectile, // Model variables, mostly projectile properties
    pub scope: &'p Scope,           // Model variables, mostly projectile properties
    pub zero_conditions: &'p Conditions,
    pub solve_conditions: &'p Conditions,
    pub time_step: Numeric,
}
impl<'p> Simulator<'p> {
    pub fn new(
        projectile: &'p Projectile,
        scope: &'p Scope,
        zero_conditions: &'p Conditions,
        solve_conditions: &'p Conditions,
        time_step: Numeric,
    ) -> Self {
        Self {
            projectile,
            scope,
            zero_conditions,
            solve_conditions,
            time_step,
        }
    }
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    fn zero_simulation(&self, zero_distance: Length) -> model::point_mass::Simulation {
        model::point_mass::Simulation::new(
            &self.projectile,
            &self.scope,
            &self.zero_conditions,
            0.0,
            zero_distance,
            self.time_step,
        )
    }
    // Create a simulation with muzzle pitch found in 'zeroin' simulation
    // Then solve for current conditions
    // Can be used for drop table, or eventually dialing in a specific distance
    fn solution_simulation(
        &self,
        zero_distance: Length,
        offset: Numeric,
    ) -> model::point_mass::Simulation {
        model::point_mass::Simulation::new(
            &self.projectile,
            &self.scope,
            &self.solve_conditions,
            match self.zero_simulation(zero_distance).zero() {
                Ok(muzzle_pitch) => muzzle_pitch + (offset / 60.0).to_radians(),
                Err(err) => panic!(err),
            },
            zero_distance,
            self.time_step,
        )
    }
    // Produce a drop table using specified range and step size
    pub fn drop_table(
        &self,
        zero_distance: Numeric,
        step: u32,
        range: u32,
        offset: Numeric,
    ) -> FloatMap<TableVal>
    {
        let mut current_step: u32 = 0; // This can overflow, not sure how to check
        self.solution_simulation(Length::Yards(zero_distance), offset)
            .into_iter()
            .do_take_while(|p| p.distance() <= Numeric::from(range))
            .filter_map(|p| {
                if p.distance() >= Numeric::from(current_step) {
                    current_step += step; // Can overflow here
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
            })
            .collect::<FloatMap<TableVal>>()
    }
}
