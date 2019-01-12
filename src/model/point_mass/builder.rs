pub use crate::model::BallisticCoefficient;

use super::*;

// Distance => (drop, windage, velocity, energy, moa, time)
pub struct SimulationBuilder<'p> {
    pub projectile: &'p Projectile, // Model variables, mostly projectile properties
    pub scope: &'p Scope,           // Model variables, mostly projectile properties
    pub zero_conditions: &'p Conditions<'p>,
    pub solve_conditions: &'p Conditions<'p>,
    pub time_step: Numeric,
}
impl<'p> SimulationBuilder<'p> {
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
    pub fn flat_simulation(&self) -> Simulation {
        Simulation::new(
            self.projectile,
            self.scope,
            self.zero_conditions,
            0.0,
            0.0,
            self.time_step,
        )
    }
    // Create a simulation with muzzle pitch found in 'zeroin' simulation
    // Then solve for current conditions
    // Can be used for drop table, or eventually dialing in a specific distance
    pub fn solve_for(
        &self,
        zero_distance: Numeric,
        zero_offset: Numeric,
        zero_tolerance: Numeric,
        pitch_offset: Numeric,
        yaw_offset: Numeric,
    ) -> Simulation {
        let zero_distance = Length::Yards(zero_distance).to_meters().to_num();
        let zero_offset = Length::Inches(zero_offset).to_meters().to_num();
        let zero_tolerance = Length::Inches(zero_tolerance).to_meters().to_num();
        let pitch_offset = (pitch_offset / 60.0).to_radians();
        let yaw_offset = (yaw_offset / 60.0).to_radians();
        Simulation::new(
            self.projectile,
            self.scope,
            self.solve_conditions,
            self.flat_simulation()
                .zero(zero_distance, zero_offset, zero_tolerance)
                .map(|muzzle_pitch| muzzle_pitch + pitch_offset)
                .expect("Zeroing Failed"),
            yaw_offset,
            self.time_step,
        )
    }
}
