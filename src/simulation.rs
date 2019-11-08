use self::BcKind::*;
use crate::{dragtables::*, util::*, Error, ErrorKind, Result};

#[macro_use]
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Simulation<'t> {
    pub(crate) flags: Flags, // Flags to enable/disable certain parts of simulation
    pub(crate) projectile: Projectile<'t>, // Use same projectile for zeroing and solving
    pub(crate) scope: Scope, // Use same scope for zeroing and solving
    pub(crate) atmosphere: Atmosphere, // Different conditions during solving
    pub(crate) wind: Wind,   // Different conditions during solving
    pub(crate) shooter: Shooter, // Different conditions during solving
    pub(crate) time_step: Numeric, // Use same timestep for zeroing and solving
}
#[derive(Debug)]
pub struct Atmosphere {
    pub(crate) temperature: Temperature, // Temperature (F)
    pub(crate) pressure: Pressure,       // Pressure (InHg)
    pub(crate) humidity: Numeric,        // Humidity (0-1)
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
    pub(crate) roll: Angle,        // Doesn't make sense, just here for consistency
    pub(crate) velocity: Velocity, // Wind Velocity (miles/hour)
}
#[derive(Debug)]
pub struct Projectile<'t> {
    pub(crate) caliber: Length,    // Caliber (inches)
    pub(crate) weight: WeightMass, // Weight (grains)
    pub(crate) bc: Bc<'t>,         // Ballistic Coefficient
    pub(crate) velocity: Velocity, // Initial velocity (ft/s)
}
#[derive(Debug)]
pub struct Bc<'t> {
    pub(crate) value: Numeric,
    pub(crate) kind: BcKind,
    pub(crate) table: Option<&'t FloatMap<Numeric>>,
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
impl Bc<'_> {
    fn init(&mut self) {
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
        self.table = Some(match self.kind {
            G1 => &G1_TABLE,
            G2 => &G2_TABLE,
            G5 => &G5_TABLE,
            G6 => &G6_TABLE,
            G7 => &G7_TABLE,
            G8 => &G8_TABLE,
            GI => &GI_TABLE,
            GS => &GS_TABLE,
        });
    }
}
#[derive(Debug)]
pub struct SimulationBuilder<'t> {
    pub(crate) builder: Simulation<'t>,
}
impl<'t> From<SimulationBuilder<'t>> for Simulation<'t> {
    fn from(other: SimulationBuilder<'t>) -> Self {
        Self { ..other.builder }
    }
}
impl<'t> From<Simulation<'t>> for SimulationBuilder<'t> {
    fn from(other: Simulation<'t>) -> Self {
        Self { builder: other }
    }
}
impl Default for SimulationBuilder<'_> {
    fn default() -> Self {
        Self {
            builder: Simulation {
                flags: Flags {
                    coriolis: true,
                    drag: true,
                    gravity: true,
                },
                projectile: Projectile {
                    caliber: Length::Inches(0.264),
                    weight: WeightMass::Grains(140.0),
                    bc: Bc {
                        value: 0.0,
                        kind: BcKind::G7,
                        table: None,
                    },
                    velocity: Velocity::Fps(2710.0),
                },
                scope: Scope {
                    yaw: Angle::Minutes(0.0),
                    pitch: Angle::Minutes(0.0),
                    roll: Angle::Degrees(0.0),
                    height: Length::Inches(1.5),
                    offset: Length::Inches(0.0),
                },
                atmosphere: Atmosphere {
                    temperature: Temperature::F(68.0),
                    pressure: Pressure::Inhg(29.92),
                    humidity: 0.0,
                },
                wind: Wind {
                    yaw: Angle::Degrees(0.0),
                    pitch: Angle::Degrees(0.0),
                    roll: Angle::Degrees(0.0),
                    velocity: Velocity::Mph(0.0),
                },
                shooter: Shooter {
                    yaw: Angle::Minutes(0.0),
                    pitch: Angle::Minutes(0.0),
                    roll: Angle::Degrees(0.0),
                    lattitude: Angle::Degrees(0.0),
                    gravity: Acceleration::Mps2(-9.806_65),
                },
                time_step: 0.000_001,
            },
        }
    }
}

impl<'t> SimulationBuilder<'t> {
    pub fn new() -> Self {
        Default::default()
    }
    // Create simulation with conditions used to find muzzle_pitch for 'zeroing'
    // Starting from flat fire pitch (0.0)
    pub fn init(self) -> Simulation<'t> {
        From::from(self)
    }
    pub fn set_bc(mut self, value: Numeric, kind: BcKind) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.projectile.bc.value = value;
            self.builder.projectile.bc.kind = kind;
            self.builder.projectile.bc.init();
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    pub fn set_time_step(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (0.0, 0.1);
        if value > min && value <= max {
            self.builder.time_step = value;
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
        }
    }

    // Atmosphere
    pub fn set_temperature(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-112.0, 122.0);
        if value >= min && value <= max {
            self.builder.atmosphere.temperature = Temperature::F(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
        }
    }
    pub fn set_pressure(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.atmosphere.pressure = Pressure::Inhg(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
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
    pub fn set_shot_angle(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-90.0, 90.0);
        if value >= min && value <= max {
            self.builder.shooter.pitch = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
        }
    }
    pub fn set_lattitude(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-90.0, 90.0);
        if value >= min && value <= max {
            self.builder.shooter.lattitude = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
        }
    }
    pub fn set_bearing(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-360.0, 360.0);
        if value >= min && value <= max {
            self.builder.shooter.yaw = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
        }
    }
    pub fn set_gravity(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_negative() {
            self.builder.shooter.gravity = Acceleration::Fps2(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::NegativeExpected(value)))
        }
    }

    // Wind
    pub fn set_wind_speed(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.wind.velocity = Velocity::Mph(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    pub fn set_wind_angle(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-360.0, 360.0);
        if value >= min && value <= max {
            self.builder.wind.yaw = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
        }
    }

    //Scope
    pub fn set_scope_height(mut self, value: Numeric) -> Self {
        self.builder.scope.height = Length::Inches(value);
        self
    }
    pub fn set_scope_offset(mut self, value: Numeric) -> Self {
        self.builder.scope.offset = Length::Inches(value);
        self
    }
    pub fn set_scope_pitch(mut self, value: Numeric) -> Self {
        self.builder.scope.pitch = Angle::Minutes(value);
        self
    }
    pub fn set_scope_yaw(mut self, value: Numeric) -> Self {
        self.builder.scope.yaw = Angle::Minutes(value);
        self
    }
    pub fn set_scope_roll(mut self, value: Numeric) -> Self {
        self.builder.scope.roll = Angle::Degrees(value);
        self
    }

    //Projectile
    pub fn set_caliber(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.projectile.caliber = Length::Inches(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    pub fn set_velocity(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.projectile.velocity = Velocity::Fps(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    pub fn set_grains(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.builder.projectile.weight = WeightMass::Grains(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
}

// Not sure how to handle/name these things yet - should be in a trait, as it's a public API
impl Simulation<'_> {
    pub fn public_air_desnity(&self) -> Numeric {
        Density::Kgpm3(self.atmosphere.rho()).to_lbpf3().to_num()
    }
    pub fn public_speed_of_sound(&self) -> Numeric {
        Velocity::Mps(self.atmosphere.speed_of_sound())
            .to_fps()
            .to_num()
    }
}
