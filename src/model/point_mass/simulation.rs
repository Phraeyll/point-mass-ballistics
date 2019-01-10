pub use crate::model::BallisticCoefficient;

use super::{iter::Output, *};

// Distance => (drop, windage, velocity, energy, moa, time)
type TableVal = (Numeric, Numeric, Numeric, Numeric, Numeric, Numeric);

pub struct SimulationBuilder<'p> {
    pub projectile: &'p Projectile, // Model variables, mostly projectile properties
    pub scope: &'p Scope,           // Model variables, mostly projectile properties
    pub zero_conditions: &'p Conditions,
    pub solve_conditions: &'p Conditions,
    pub zero_distance: Numeric,
    pub time_step: Numeric,
}
impl<'p> SimulationBuilder<'p> {
    pub fn new(
        projectile: &'p Projectile,
        scope: &'p Scope,
        zero_conditions: &'p Conditions,
        solve_conditions: &'p Conditions,
        zero_distance: Numeric,
        time_step: Numeric,
    ) -> Self {
        Self {
            projectile,
            scope,
            zero_conditions,
            solve_conditions,
            zero_distance,
            time_step,
        }
    }
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    fn zero_simulation(&self) -> Simulation {
        Simulation::new(
            &self.projectile,
            &self.scope,
            &self.zero_conditions,
            0.0,
            self.zero_distance,
            self.time_step,
        )
    }
    // Create a simulation with muzzle pitch found in 'zeroin' simulation
    // Then solve for current conditions
    // Can be used for drop table, or eventually dialing in a specific distance
    fn solution_simulation(&self, offset: Numeric) -> Simulation {
        Simulation::new(
            &self.projectile,
            &self.scope,
            &self.solve_conditions,
            match self.zero_simulation().zero() {
                Ok(muzzle_pitch) => muzzle_pitch + (offset / 60.0).to_radians(),
                Err(err) => panic!(err),
            },
            self.zero_distance,
            self.time_step,
        )
    }
    // Produce a drop table using specified range and step size
    pub fn drop_table(&self, step: u32, range: u32, offset: Numeric) -> FloatMap<TableVal> {
        let sim = self.solution_simulation(offset);
        let mut iter = sim.iter().fuse();
        (0..=range)
            .step_by(step as usize)
            .filter_map(|current_step| {
                iter.by_ref()
                    .find(|p| p.distance() >= Numeric::from(current_step))
                    .map(|p| {
                        (
                            p.distance(), // Key
                            (
                                p.elevation(),
                                p.windage(),
                                p.velocity(),
                                p.energy(),
                                p.moa(),
                                p.time(),
                            ), // Value
                        )
                    })
            })
            .collect::<FloatMap<_>>()
    }
}
