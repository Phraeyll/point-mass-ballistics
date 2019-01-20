use super::*;

// Distance => (drop, windage, velocity, energy, moa, time)
pub struct SimulationBuilder {
    pub projectile: Projectile, // Model variables, mostly projectile properties
    pub scope: Scope,           // Model variables, mostly projectile properties
    pub zero_conditions: Conditions,
    pub solve_conditions: Conditions,
    pub time_step: Time,
}
impl Default for SimulationBuilder {
    fn default() -> Self {
        Self {
            projectile: Projectile::default(),
            scope: Scope::default(),
            zero_conditions: Conditions::default(),
            solve_conditions: Conditions::default(),
            time_step: Time::Seconds(0.000_001),
        }
    }
}
impl SimulationBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn projectile(mut self, projectile: Projectile) -> Self {
        self.projectile = projectile;
        self
    }
    pub fn scope(mut self, scope: Scope) -> Self {
        self.scope = scope;
        self
    }
    pub fn zero_conditions(mut self, conditions: Conditions) -> Self {
        self.zero_conditions = conditions;
        self
    }
    pub fn solve_conditions(mut self, conditions: Conditions) -> Self {
        self.solve_conditions = conditions;
        self
    }
    pub fn time_step(mut self, time_step: Numeric) -> Self {
        self.time_step = Time::Seconds(time_step);
        self
    }
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    pub fn flat(&self, pitch_offset: Numeric, yaw_offset: Numeric) -> Simulation {
        let pitch_offset = Angle::Minutes(pitch_offset);
        let yaw_offset = Angle::Minutes(-yaw_offset); // Invert this number, since +90 is left in trig calculations
        Simulation::new(
            &self.projectile,
            &self.scope,
            &self.zero_conditions,
            self.time_step,
            pitch_offset,
            yaw_offset,
        )
    }
    // Create a simulation with muzzle pitch found in 'zeroin' simulation
    // Then solve for current conditions
    // Can be used for drop table, or eventually dialing in a specific distance
    pub fn solve_for(
        &self,
        zero_distance: Numeric,
        zero_elevation_offset: Numeric,
        zero_windage_offset: Numeric,
        zero_tolerance: Numeric,
        pitch_offset: Numeric,
        yaw_offset: Numeric,
    ) -> Simulation {
        let zero_distance = Length::Yards(zero_distance);
        let zero_elevation_offset = Length::Inches(zero_elevation_offset);
        let zero_windage_offset = Length::Inches(zero_windage_offset);
        let zero_tolerance = Length::Inches(zero_tolerance);
        let pitch_offset = Angle::Minutes(pitch_offset);
        let yaw_offset = Angle::Minutes(-yaw_offset); // Invert this number, since +90 is left in trig calculations
        let (found_pitch, found_yaw) = self
            .flat(0.0, 0.0)
            .zero(
                zero_distance,
                zero_elevation_offset,
                zero_windage_offset,
                zero_tolerance,
            )
            .map(|(found_pitch, found_yaw)| {
                (
                    Angle::Radians(
                        found_pitch.to_radians().to_num() + pitch_offset.to_radians().to_num(),
                    ),
                    Angle::Radians(
                        found_yaw.to_radians().to_num() + yaw_offset.to_radians().to_num(),
                    ),
                )
            })
            .expect("solve_for");
        Simulation::new(
            &self.projectile,
            &self.scope,
            &self.solve_conditions,
            self.time_step,
            found_pitch,
            found_yaw,
        )
    }
}
