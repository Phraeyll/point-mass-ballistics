use super::*;

use std::error::Error as StdError;
use std::fmt;
use std::fmt::Display as StdDisplay;
use std::result;
use std::str;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error(Box::new(kind))
    }
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Input,
    PositiveExpected(Numeric),
    OutOfRange(Numeric, Numeric),
}

impl StdDisplay for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::Input => write!(formatter, "Input Error"),
            ErrorKind::PositiveExpected(ref err) => {
                write!(formatter, "Positive Expected Error: {}", err)
            }
            ErrorKind::OutOfRange(ref start, ref end) => write!(
                formatter,
                "Within Range Expected Error: {} - {}",
                start, end
            ),
        }
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        match *self.0 {
            ErrorKind::Input => "Invalid inputs",
            ErrorKind::PositiveExpected(..) => "Number needs to be positive greater than 0",
            ErrorKind::OutOfRange(..) => "Numer needs to be within range",
        }
    }
}

pub struct Solver {
    pub projectile: Projectile, // Use same projectile for zeroing and solving
    pub scope: Scope,           // Use same scope for zeroing and solving
    pub zero_conditions: Conditions, // Different conditions during zeroing
    pub solve_conditions: Conditions, // Different conditions during solving
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
        }
    }
}

pub trait SimulationBuilder<'a> {
    type Simulation;
    fn new() -> Self;
    fn projectile(self, projectile: Projectile) -> Self;
    fn scope(self, scope: Scope) -> Self;
    fn zero_conditions(self, conditions: Conditions) -> Self;
    fn solve_conditions(self, conditions: Conditions) -> Self;
    fn time_step(self, time_step: Numeric) -> Self;
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
            self.time_step,
            found_pitch,
            found_yaw,
        )
    }
    fn new() -> Self {
        Self::default()
    }
    fn projectile(mut self, projectile: Projectile) -> Self {
        self.projectile = projectile;
        self
    }
    fn scope(mut self, scope: Scope) -> Self {
        self.scope = scope;
        self
    }
    fn zero_conditions(mut self, conditions: Conditions) -> Self {
        self.zero_conditions = conditions;
        self
    }
    fn solve_conditions(mut self, conditions: Conditions) -> Self {
        self.solve_conditions = conditions;
        self
    }
    fn time_step(mut self, time_step: Numeric) -> Self {
        self.time_step = Time::Seconds(time_step);
        self
    }
}

pub trait ProjectileBuilder {
    fn new() -> Self;
    fn with_velocity(self, velocity: Numeric) -> Self;
    fn with_grains(self, grains: Numeric) -> Self;
    fn with_caliber(self, caliber: Numeric) -> Self;
    fn with_bc(self, bc: BallisticCoefficient) -> Self;
}
impl ProjectileBuilder for Projectile {
    fn new() -> Self {
        Self::default()
    }
    fn with_velocity(mut self, velocity: Numeric) -> Self {
        self.velocity = Velocity::Fps(velocity);
        self
    }
    fn with_grains(mut self, grains: Numeric) -> Self {
        self.weight = WeightMass::Grains(grains);
        self
    }
    fn with_caliber(mut self, caliber: Numeric) -> Self {
        self.caliber = Length::Inches(caliber);
        self
    }
    fn with_bc(mut self, bc: BallisticCoefficient) -> Self {
        self.bc = bc;
        self
    }
}
pub trait ConditionsBuilder {
    fn new() -> Self;
    fn with_temperature(self, temperature: Numeric) -> Self;
    fn with_pressure(self, pressure: Numeric) -> Self;
    fn with_humidity(self, humidity: Numeric) -> Self;
    fn with_wind_speed(self, velocity: Numeric) -> Self;
    fn with_wind_angle(self, yaw: Numeric) -> Self;
    fn with_shot_angle(self, line_of_sight: Numeric) -> Self;
    fn with_lattitude(self, lattitude: Numeric) -> Self;
    fn with_bearing(self, azimuth: Numeric) -> Self;
}
impl ConditionsBuilder for Conditions {
    fn new() -> Self {
        Self::default()
    }
    fn with_temperature(mut self, temperature: Numeric) -> Self {
        self.atmosphere.temperature = Temperature::F(temperature);
        self
    }
    fn with_pressure(mut self, pressure: Numeric) -> Self {
        self.atmosphere.pressure = Pressure::Inhg(pressure);
        self
    }
    fn with_humidity(mut self, humidity: Numeric) -> Self {
        self.atmosphere.humidity = humidity;
        self
    }
    fn with_wind_speed(mut self, velocity: Numeric) -> Self {
        self.wind.velocity = Velocity::Mph(velocity);
        self
    }
    fn with_wind_angle(mut self, yaw: Numeric) -> Self {
        self.wind.yaw = Angle::Degrees(yaw);
        self
    }
    fn with_shot_angle(mut self, line_of_sight: Numeric) -> Self {
        self.other.line_of_sight = Angle::Degrees(line_of_sight);
        self
    }
    fn with_lattitude(mut self, lattitude: Numeric) -> Self {
        self.other.lattitude = Angle::Degrees(lattitude);
        self
    }
    fn with_bearing(mut self, azimuth: Numeric) -> Self {
        self.other.azimuth = Angle::Degrees(azimuth);
        self
    }
}

pub trait ScopeBuilder {
    fn new() -> Self;
    fn with_height(self, height: Numeric) -> Result<Self>
    where
        Self: std::marker::Sized;
}
impl ScopeBuilder for Scope {
    fn new() -> Self {
        Self::default()
    }
    fn with_height(mut self, height: Numeric) -> Result<Self> {
        if height.is_sign_positive() {
            self.height = Length::Inches(height);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::Input))
        }
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
