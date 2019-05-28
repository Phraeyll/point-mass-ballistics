use crate::{
    model::core::{ShooterAdjuster, SimulationBuilder},
    util::*,
};

const GRAVITY: Numeric = -9.806_65; // Local gravity in m/s

pub fn default_gravity() -> Acceleration {
    Acceleration::Mps2(GRAVITY)
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
pub struct ShooterBuilder {
    pub yaw: Angle,       // Bearing (0 North, 90 East) (degrees) (Coriolis/Eotvos Effect)
    pub pitch: Angle,     // Line of Sight angle (degrees)
    pub roll: Angle,      // Roll relative to shooters position, ie, scope alligned with rifle
    pub lattitude: Angle, // Lattitude (Coriolis/Eotvos Effect)
    pub gravity: Acceleration, // Gravity (m/s^2)
}
impl From<ShooterBuilder> for Shooter {
    fn from(other: ShooterBuilder) -> Self {
        Self {
            yaw: other.yaw,
            pitch: other.pitch,
            roll: other.roll,
            lattitude: other.lattitude,
            gravity: other.gravity,
        }
    }
}
impl From<Shooter> for ShooterBuilder {
    fn from(other: Shooter) -> Self {
        Self {
            yaw: other.yaw,
            pitch: other.pitch,
            roll: other.roll,
            lattitude: other.lattitude,
            gravity: other.gravity,
        }
    }
}
impl Default for ShooterBuilder {
    fn default() -> Self {
        Self {
            yaw: Angle::Minutes(0.0),
            pitch: Angle::Minutes(0.0),
            roll: Angle::Degrees(0.0),
            lattitude: Angle::Degrees(0.0),
            gravity: default_gravity(),
        }
    }
}
impl ShooterAdjuster for SimulationBuilder {
    fn set_shot_angle(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-90.0, 90.0);
        if value >= min && value <= max {
            self.shooter.pitch = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
        }
    }
    fn set_lattitude(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-90.0, 90.0);
        if value >= min && value <= max {
            self.shooter.lattitude = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
        }
    }
    fn set_bearing(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-360.0, 360.0);
        if value >= min && value <= max {
            self.shooter.yaw = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange { min, max }))
        }
    }
    fn set_gravity(mut self, value: Numeric) -> Result<Self> {
        self.shooter.gravity = Acceleration::Fps2(value);
        Ok(self)
    }
}
