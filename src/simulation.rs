use self::BcKind::*;
use crate::{
    dragtables::*,
    util::{
        celsius, fahrenheit, foot_per_second, grain, inch, inch_of_mercury, kelvin, kilogram,
        meter, meter_per_second, meter_per_second_squared, mile_per_hour, pascal, radian, second,
        Acceleration, Angle, FloatMap, Length, Mass, Numeric, Pressure, ThermodynamicTemperature,
        Time, Velocity, FRAC_PI_2, PI,
    },
    Error, ErrorKind, Result,
};

use std::str::FromStr;

use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Simulation {
    pub(crate) flags: Flags, // Flags to enable/disable certain parts of simulation
    pub(crate) projectile: Projectile, // Use same projectile for zeroing and solving
    pub(crate) scope: Scope, // Use same scope for zeroing and solving
    pub(crate) atmosphere: Atmosphere, // Different conditions during solving
    pub(crate) wind: Wind,   // Different conditions during solving
    pub(crate) shooter: Shooter, // Different conditions during solving
    pub(crate) time_step: Time, // Use same timestep for zeroing and solving
}
#[derive(Debug)]
pub struct Atmosphere {
    pub(crate) temperature: ThermodynamicTemperature, // Temperature (F)
    pub(crate) pressure: Pressure,                    // Pressure (InHg)
    pub(crate) humidity: Numeric,                     // Humidity (0-1)
}
#[derive(Debug)]
pub struct Flags {
    pub(crate) coriolis: bool, // Whether or not to calculate coriolis/eotvos effect
    pub(crate) drag: bool,     // Whether or not to calculate drag
    pub(crate) gravity: bool,  // Whether or not to calculate gravity
}
#[derive(Debug)]
pub struct Scope {
    pub(crate) yaw: Angle,
    pub(crate) pitch: Angle,
    pub(crate) roll: Angle,    // Scope Roll (Cant) (Degrees)
    pub(crate) height: Length, // Scope Height (inches)
    pub(crate) offset: Length, // Scope Offset Windage (left/right boreline) (inches)
}
#[derive(Debug)]
pub struct Shooter {
    pub(crate) yaw: Angle, // Bearing (0 North, 90 East) (degrees) (Coriolis/Eotvos Effect)
    pub(crate) pitch: Angle, // Line of Sight angle (degrees)
    pub(crate) roll: Angle, // Roll relative to shooters position, ie, scope alligned with rifle
    pub(crate) lattitude: Angle, // Lattitude (Coriolis/Eotvos Effect)
    pub(crate) gravity: Acceleration, // Gravity (m/s^2)
}
#[derive(Debug)]
pub struct Wind {
    pub(crate) yaw: Angle,         // Wind Angle (degrees)
    pub(crate) pitch: Angle,       // Wind Pitch (degrees)
    pub(crate) roll: Angle,        // Doesn make sense, just here for consistency
    pub(crate) velocity: Velocity, // Wind Velocity (miles/hour)
}
#[derive(Debug)]
pub struct Projectile {
    pub(crate) caliber: Length,    // Caliber (inches)
    pub(crate) weight: Mass,       // Weight (grains)
    pub(crate) bc: Bc,             // Ballistic Coefficient
    pub(crate) velocity: Velocity, // Initial velocity (ft/s)
}
#[derive(Debug)]
pub struct Bc {
    pub(crate) value: Numeric,
    pub(crate) kind: BcKind,
}
#[derive(Debug, Copy, Clone)]
pub enum BcKind {
    G1,
    G2,
    G5,
    G6,
    G7,
    G8,
    GI,
    GS,
}
impl FromStr for BcKind {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "G1" => Ok(G1),
            "G2" => Ok(G2),
            "G5" => Ok(G5),
            "G6" => Ok(G6),
            "G7" => Ok(G7),
            "G8" => Ok(G8),
            "GI" => Ok(GI),
            "GS" => Ok(GS),
            _ => Err(Error::new(ErrorKind::InvalidBcKind(s.to_string()))),
        }
    }
}
impl Bc {
    pub(crate) fn table(&self) -> &'static FloatMap<Numeric> {
        lazy_static! {
            static ref G1_TABLE: FloatMap<Numeric> = g1::init();
            static ref G2_TABLE: FloatMap<Numeric> = g2::init();
            static ref G5_TABLE: FloatMap<Numeric> = g5::init();
            static ref G6_TABLE: FloatMap<Numeric> = g6::init();
            static ref G7_TABLE: FloatMap<Numeric> = g7::init();
            static ref G8_TABLE: FloatMap<Numeric> = g8::init();
            static ref GI_TABLE: FloatMap<Numeric> = gi::init();
            static ref GS_TABLE: FloatMap<Numeric> = gs::init();
        };
        match self.kind {
            G1 => &G1_TABLE,
            G2 => &G2_TABLE,
            G5 => &G5_TABLE,
            G6 => &G6_TABLE,
            G7 => &G7_TABLE,
            G8 => &G8_TABLE,
            GI => &GI_TABLE,
            GS => &GS_TABLE,
        }
    }
}
#[derive(Debug)]
pub struct SimulationBuilder {
    pub(crate) builder: Simulation,
}
impl From<SimulationBuilder> for Simulation {
    fn from(other: SimulationBuilder) -> Self {
        Self { ..other.builder }
    }
}
impl From<Simulation> for SimulationBuilder {
    fn from(other: Simulation) -> Self {
        Self { builder: other }
    }
}
impl Default for SimulationBuilder {
    fn default() -> Self {
        Self {
            builder: Simulation {
                flags: Flags {
                    coriolis: true,
                    drag: true,
                    gravity: true,
                },
                projectile: Projectile {
                    caliber: Length::new::<inch>(0.264),
                    weight: Mass::new::<grain>(140.0),
                    bc: Bc {
                        value: 0.305,
                        kind: BcKind::G7,
                    },
                    velocity: Velocity::new::<foot_per_second>(2710.0),
                },
                scope: Scope {
                    yaw: Angle::new::<radian>(0.0),
                    pitch: Angle::new::<radian>(0.0),
                    roll: Angle::new::<radian>(0.0),
                    height: Length::new::<inch>(1.5),
                    offset: Length::new::<inch>(0.0),
                },
                atmosphere: Atmosphere {
                    temperature: ThermodynamicTemperature::new::<fahrenheit>(68.0),
                    pressure: Pressure::new::<inch_of_mercury>(29.92),
                    humidity: 0.0,
                },
                wind: Wind {
                    yaw: Angle::new::<radian>(0.0),
                    pitch: Angle::new::<radian>(0.0),
                    roll: Angle::new::<radian>(0.0),
                    velocity: Velocity::new::<mile_per_hour>(0.0),
                },
                shooter: Shooter {
                    yaw: Angle::new::<radian>(0.0),
                    pitch: Angle::new::<radian>(0.0),
                    roll: Angle::new::<radian>(0.0),
                    lattitude: Angle::new::<radian>(0.0),
                    gravity: Acceleration::new::<meter_per_second_squared>(-9.806_65),
                },
                time_step: Time::new::<second>(0.000_001),
            },
        }
    }
}

impl SimulationBuilder {
    pub fn new() -> Self {
        Default::default()
    }
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    pub fn init(self) -> Simulation {
        From::from(self)
    }
    pub fn set_time_step(mut self, value: Time) -> Result<Self> {
        let min = Time::new::<second>(0.0);
        let max = Time::new::<second>(0.1);
        if value > min && value <= max {
            self.builder.time_step = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange {
                min: min.get::<second>(),
                max: max.get::<second>(),
            }))
        }
    }

    // Atmosphere
    pub fn set_temperature(mut self, value: ThermodynamicTemperature) -> Result<Self> {
        let min = ThermodynamicTemperature::new::<celsius>(-80.0);
        let max = ThermodynamicTemperature::new::<celsius>(50.0);
        if value >= min && value <= max {
            self.builder.atmosphere.temperature = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange {
                min: min.get::<kelvin>(),
                max: max.get::<kelvin>(),
            }))
        }
    }
    pub fn set_pressure(mut self, value: Pressure) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.atmosphere.pressure = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(
                value.get::<pascal>(),
            )))
        }
    }
    pub fn set_humidity(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (0.0, 1.0);
        if value >= min && value <= max {
            self.builder.atmosphere.humidity = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
        }
    }

    // Flags
    pub fn use_coriolis(mut self, value: bool) -> Self {
        self.builder.flags.coriolis = value;
        self
    }
    pub fn use_drag(mut self, value: bool) -> Self {
        self.builder.flags.drag = value;
        self
    }
    pub fn use_gravity(mut self, value: bool) -> Self {
        self.builder.flags.gravity = value;
        self
    }

    // Shooter
    pub fn set_shot_angle(mut self, value: Angle) -> Result<Self> {
        let min = Angle::new::<radian>(-FRAC_PI_2);
        let max = Angle::new::<radian>(FRAC_PI_2);
        if value >= min && value <= max {
            self.builder.shooter.pitch = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange {
                min: min.get::<radian>(),
                max: max.get::<radian>(),
            }))
        }
    }
    pub fn set_lattitude(mut self, value: Angle) -> Result<Self> {
        let min = Angle::new::<radian>(-FRAC_PI_2);
        let max = Angle::new::<radian>(FRAC_PI_2);
        if value >= min && value <= max {
            self.builder.shooter.lattitude = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange {
                min: min.get::<radian>(),
                max: max.get::<radian>(),
            }))
        }
    }
    pub fn set_bearing(mut self, value: Angle) -> Result<Self> {
        let min = Angle::new::<radian>(-2.0 * PI);
        let max = Angle::new::<radian>(2.0 * PI);
        if value >= min && value <= max {
            self.builder.shooter.yaw = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange {
                min: min.get::<radian>(),
                max: max.get::<radian>(),
            }))
        }
    }
    pub fn set_gravity(mut self, value: Acceleration) -> Result<Self> {
        if value.is_sign_negative() {
            self.builder.shooter.gravity = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::NegativeExpected(
                value.get::<meter_per_second_squared>(),
            )))
        }
    }

    // Wind
    pub fn set_wind_speed(mut self, value: Velocity) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.wind.velocity = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(
                value.get::<meter_per_second>(),
            )))
        }
    }
    pub fn set_wind_angle(mut self, value: Angle) -> Result<Self> {
        let min = Angle::new::<radian>(-2.0 * PI);
        let max = Angle::new::<radian>(2.0 * PI);
        if value >= min && value <= max {
            self.builder.wind.yaw = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange {
                min: min.get::<radian>(),
                max: max.get::<radian>(),
            }))
        }
    }

    //Scope
    pub fn set_scope_height(mut self, value: Length) -> Self {
        self.builder.scope.height = value;
        self
    }
    pub fn set_scope_offset(mut self, value: Length) -> Self {
        self.builder.scope.offset = value;
        self
    }
    pub fn set_scope_pitch(mut self, value: Angle) -> Self {
        self.builder.scope.pitch = value;
        self
    }
    pub fn set_scope_yaw(mut self, value: Angle) -> Self {
        self.builder.scope.yaw = value;
        self
    }
    pub fn set_scope_roll(mut self, value: Angle) -> Self {
        self.builder.scope.roll = value;
        self
    }

    //Projectile
    pub fn set_caliber(mut self, value: Length) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.projectile.caliber = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(
                value.get::<meter>(),
            )))
        }
    }
    pub fn set_velocity(mut self, value: Velocity) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.projectile.velocity = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(
                value.get::<meter_per_second>(),
            )))
        }
    }
    pub fn set_mass(mut self, value: Mass) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.projectile.weight = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(
                value.get::<kilogram>(),
            )))
        }
    }
    pub fn set_bc_value(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.projectile.bc.value = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    pub fn set_bc_kind(mut self, value: BcKind) -> Result<Self> {
        self.builder.projectile.bc.kind = value;
        Ok(self)
    }
}
