pub use BallisticCoefficientKind::*;

use crate::error::{Error, ErrorKind, Result};
use nalgebra::Vector3;

use super::*;

pub struct Solver {
    pub projectile: Projectile, // Use same projectile for zeroing and solving
    pub scope: Scope,           // Use same scope for zeroing and solving
    pub zero_conditions: Conditions, // Different conditions during zeroing
    pub solve_conditions: Conditions, // Different conditions during solving
    pub flags: Flags,           // Flags to enable/disable certain parts of simulation
    pub time_step: Time,        // Use same timestep for zeroing and solving
}
impl Default for Solver {
    fn default() -> Self {
        Self {
            projectile: Projectile::default(),
            scope: Scope::default(),
            zero_conditions: Conditions::default(),
            solve_conditions: Conditions::default(),
            time_step: Time::Seconds(0.000_001),
            flags: Flags::default(),
        }
    }
}

pub trait SimulationBuilder<'a> {
    type Simulation;
    fn new() -> Self;
    fn projectile(self, value: Projectile) -> Self;
    fn scope(self, value: Scope) -> Self;
    fn zero_conditions(self, value: Conditions) -> Self;
    fn solve_conditions(self, value: Conditions) -> Self;
    fn time_step(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn flags(self, value: Flags) -> Self;
    fn using_zero_conditions(
        &'a self,
        pitch_offset: Numeric,
        yaw_offset: Numeric,
    ) -> Self::Simulation;
    fn solve_for(
        &'a self,
        zero_distance: Numeric,
        zero_elevation_offset: Numeric,
        zero_windage_offset: Numeric,
        zero_tolerance: Numeric,
        pitch_offset: Numeric,
        yaw_offset: Numeric,
    ) -> Self::Simulation;
}
impl<'a> SimulationBuilder<'a> for Solver {
    type Simulation = Simulation<'a>;
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    fn using_zero_conditions(
        &'a self,
        pitch_offset: Numeric,
        yaw_offset: Numeric,
    ) -> Self::Simulation {
        let pitch_offset = Angle::Minutes(pitch_offset);
        let yaw_offset = Angle::Minutes(-yaw_offset); // Invert this number, since +90 is left in trig calculations
        Simulation::new(
            &self.projectile,
            &self.scope,
            &self.zero_conditions,
            &self.flags,
            self.time_step,
            pitch_offset,
            yaw_offset,
        )
    }
    // Create a simulation with muzzle pitch found in 'zeroin' simulation
    // Then solve for current conditions
    // Can be used for drop table, or eventually dialing in a specific distance
    fn solve_for(
        &'a self,
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

        // Attempt to zero to given parameters, accounting for different conditions
        // Start with 0.0 pitch and 0.0 yaw
        // Then use found pitch/yaw for this simulation
        let (found_pitch, found_yaw) = self
            .using_zero_conditions(0.0, 0.0)
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
            &self.flags,
            self.time_step,
            found_pitch,
            found_yaw,
        )
    }
    fn new() -> Self {
        Self::default()
    }
    fn flags(mut self, value: Flags) -> Self {
        self.flags = value;
        self
    }
    fn projectile(mut self, value: Projectile) -> Self {
        self.projectile = value;
        self
    }
    fn scope(mut self, value: Scope) -> Self {
        self.scope = value;
        self
    }
    fn zero_conditions(mut self, value: Conditions) -> Self {
        self.zero_conditions = value;
        self
    }
    fn solve_conditions(mut self, value: Conditions) -> Self {
        self.solve_conditions = value;
        self
    }
    fn time_step(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (0.0, 0.1);
        if value > min && value <= max {
            self.time_step = Time::Seconds(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
}

pub trait FlagsBuilder {
    fn new() -> Self;
    fn enable_coriolis(self, value: bool) -> Self;
    fn enable_drag(self, value: bool) -> Self;
    fn enable_gravity(self, value: bool) -> Self;
}
impl FlagsBuilder for Flags {
    fn new() -> Self {
        Self::default()
    }
    fn enable_coriolis(mut self, value: bool) -> Self {
        self.use_coriolis = value;
        self
    }
    fn enable_drag(mut self, value: bool) -> Self {
        self.use_drag = value;
        self
    }
    fn enable_gravity(mut self, value: bool) -> Self {
        self.use_gravity = value;
        self
    }
}

// Unwrap BC and create associated drag table
pub trait BallisticCoefficientBuilder {
    fn new(value: Numeric, kind: BallisticCoefficientKind) -> Result<Self>
    where Self: Sized;
}
impl BallisticCoefficientBuilder for BallisticCoefficient {
    fn new(value: Numeric, kind: BallisticCoefficientKind) -> Result<Self> {
        if value.is_sign_positive() {
            Ok(Self {
                value,
                kind,
                table: match kind {
                    G1 => g1::init(),
                    G2 => g2::init(),
                    G5 => g5::init(),
                    G6 => g6::init(),
                    G7 => g7::init(),
                    G8 => g8::init(),
                    GI => gi::init(),
                    GS => gs::init(),
                },
            })
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
}

pub trait ProjectileBuilder {
    fn new() -> Self;
    fn with_velocity(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;

    fn with_grains(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;

    fn with_caliber(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;

    fn with_bc(self, value: BallisticCoefficient) -> Self;
}
impl ProjectileBuilder for Projectile {
    fn new() -> Self {
        Self::default()
    }
    fn with_velocity(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.velocity = Velocity::Fps(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn with_grains(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.weight = WeightMass::Grains(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn with_caliber(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.caliber = Length::Inches(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn with_bc(mut self, value: BallisticCoefficient) -> Self {
        self.bc = value;
        self
    }
}

pub trait ScopeBuilder {
    fn new() -> Self;
    fn with_height(self, value: Numeric) -> Self;
    fn with_offset(self, value: Numeric) -> Self;
}
impl ScopeBuilder for Scope {
    fn new() -> Self {
        Self::default()
    }
    fn with_height(mut self, value: Numeric) -> Self {
        self.height = Length::Inches(value);
        self
    }
    fn with_offset(mut self, value: Numeric) -> Self {
        self.offset = Length::Inches(value);
        self
    }
}

pub trait ConditionsBuilder {
    fn new() -> Self;
    fn with_temperature(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_pressure(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_humidity(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_wind_speed(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_wind_angle(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_shot_angle(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_lattitude(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_bearing(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;
    fn with_gravity(self, value: Numeric) -> Self;
}
impl ConditionsBuilder for Conditions {
    fn new() -> Self {
        Self::default()
    }
    fn with_temperature(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-112.0, 122.0);
        if value >= min && value <= max {
            self.atmosphere.temperature = Temperature::F(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn with_pressure(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.atmosphere.pressure = Pressure::Inhg(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn with_humidity(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (0.0, 1.0);
        if value >= min && value <= max {
            self.atmosphere.humidity = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn with_wind_speed(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.wind.velocity = Velocity::Mph(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn with_wind_angle(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-360.0, 360.0);
        if value >= min && value <= max {
            self.wind.yaw = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn with_shot_angle(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-90.0, 90.0);
        if value >= min && value <= max {
            self.other.line_of_sight = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn with_lattitude(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-90.0, 90.0);
        if value >= min && value <= max {
            self.other.lattitude = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn with_bearing(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-360.0, 360.0);
        if value >= min && value <= max {
            self.other.azimuth = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn with_gravity(mut self, value: Numeric) -> Self {
        self.other.gravity = Acceleration::Fps2(value);
        self
    }
}

pub trait Output {
    fn time(&self) -> Numeric;
    fn velocity(&self) -> Numeric;
    fn energy(&self) -> Numeric;
    fn distance(&self) -> Numeric;
    fn elevation(&self) -> Numeric;
    fn windage(&self) -> Numeric;
    fn moa(&self) -> Numeric;
    fn vertical_moa(&self, tolerance: Numeric) -> Numeric;
    fn horizontal_moa(&self, tolerance: Numeric) -> Numeric;
}
// Hard coded Imperial units for now - need to use better library for this eventually
impl Output for Packet<'_> {
    fn time(&self) -> Numeric {
        Time::Seconds(self.time).to_seconds().to_num()
    }
    fn velocity(&self) -> Numeric {
        Velocity::Mps(self.velocity.norm()).to_fps().to_num()
    }
    fn energy(&self) -> Numeric {
        Energy::Joules(self.simulation.projectile.mass() * self.velocity.norm().powf(2.0) / 2.0)
            .to_ftlbs()
            .to_num()
    }
    // Positions relative to line of sight (shooter_pitch)
    fn distance(&self) -> Numeric {
        Length::Meters(self.relative_position().x)
            .to_yards()
            .to_num()
    }
    fn elevation(&self) -> Numeric {
        Length::Meters(self.relative_position().y)
            .to_inches()
            .to_num()
    }
    fn windage(&self) -> Numeric {
        Length::Meters(self.relative_position().z)
            .to_inches()
            .to_num()
    }
    fn moa(&self) -> Numeric {
        Angle::Radians(self.relative_position().angle(&Vector3::x_axis()))
            .to_minutes()
            .to_num()
    }
    fn vertical_moa(&self, tolerance: Numeric) -> Numeric {
        self.offset_vertical_moa(Length::Inches(0.0), Length::Inches(tolerance))
            .to_minutes()
            .to_num()
    }
    fn horizontal_moa(&self, tolerance: Numeric) -> Numeric {
        self.offset_horizontal_moa(Length::Inches(0.0), Length::Inches(tolerance))
            .to_minutes()
            .to_num()
    }
}
