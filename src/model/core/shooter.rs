use nalgebra::Vector3;

use crate::model::core::{ShooterBuilder, SimulationBuilder};
use crate::util::*;

use std::ops::Mul;

const ANGULAR_VELOCITY_EARTH: Numeric = 0.000_072_921_159; // Angular velocity of earth, (radians)
const GRAVITY: Numeric = -9.806_65; // Local gravity in m/s

#[derive(Debug)]
pub struct Shooter {
    yaw: Angle,            // Bearing (0 North, 90 East) (degrees) (Coriolis/Eotvos Effect)
    pitch: Angle,          // Line of Sight angle (degrees)
    roll: Angle,           // Roll relative to shooters position, ie, scope alligned with rifle
    lattitude: Angle,      // Lattitude (Coriolis/Eotvos Effect)
    gravity: Acceleration, // Gravity (m/s^2)
}

impl Default for Shooter {
    fn default() -> Self {
        Self {
            yaw: Angle::Radians(0.0),
            pitch: Angle::Radians(0.0),
            roll: Angle::Radians(0.0),
            lattitude: Angle::Radians(0.0),
            gravity: Acceleration::Mps2(GRAVITY),
        }
    }
}

impl ShooterBuilder for SimulationBuilder {
    fn set_shot_angle(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-90.0, 90.0);
        if value >= min && value <= max {
            self.shooter.pitch = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn set_lattitude(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-90.0, 90.0);
        if value >= min && value <= max {
            self.shooter.lattitude = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn set_bearing(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-360.0, 360.0);
        if value >= min && value <= max {
            self.shooter.yaw = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
    fn set_gravity(mut self, value: Numeric) -> Result<Self> {
        self.shooter.gravity = Acceleration::Fps2(value);
        Ok(self)
    }
}
impl Shooter {
    pub(crate) fn gravity(&self) -> Vector3<Numeric> {
        self.gravity.to_mps2().to_num().mul(Vector3::y())
    }
    // Flip, since circle functions rotate counter-clockwise,
    // 90 degrees is east by compass bearing, but west(left) in trig
    //        (0)
    //         ^
    //         |
    // (+90) <---> (-90)
    //         |
    //         v
    //       (180)
    //
    //  {after negation(-)}
    //
    //        (0)
    //         ^
    //         |
    // (-90) <---> (+90)
    //         |
    //         v
    //       (180)
    pub(crate) fn yaw(&self) -> Angle {
        -self.yaw
    }
    pub(crate) fn pitch(&self) -> Angle {
        self.pitch
    }
    pub(crate) fn roll(&self) -> Angle {
        -self.roll
    }
    // Angular velocity vector of earth, at current lattitude
    // Can be thought of as vector from center of earth, pointing
    // to lines of lattitude.  Maximum effect at +/-90 degrees (poles)
    pub(crate) fn omega(&self) -> Vector3<Numeric> {
        ANGULAR_VELOCITY_EARTH
            .mul(Vector3::x())
            .pivot_z(self.lattitude)
    }
}
