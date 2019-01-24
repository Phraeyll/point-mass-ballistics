use nalgebra::Vector3;

use crate::model::core::{SimulationBuilder, WindAdjuster};
use crate::util::*;

use std::ops::Mul;

#[derive(Debug)]
pub struct Wind {
    pub(crate) yaw: Angle,         // Wind Angle (degrees)
    pub(crate) pitch: Angle,       // Wind Pitch (degrees)
    pub(crate) roll: Angle,        // Doesn't make sense, just here for consistency
    pub(crate) velocity: Velocity, // Wind Velocity (miles/hour)
}
#[derive(Debug)]
pub struct WindBuilder {
    pub yaw: Angle,         // Wind Angle (degrees)
    pub pitch: Angle,       // Wind Pitch (degrees)
    pub roll: Angle,        // Doesn't make sense, just here for consistency
    pub velocity: Velocity, // Wind Velocity (miles/hour)
}
impl From<WindBuilder> for Wind {
    fn from(other: WindBuilder) -> Self {
        Self {
            yaw: other.yaw,
            pitch: other.pitch,
            roll: other.roll,
            velocity: other.velocity,
        }
    }
}
impl From<Wind> for WindBuilder {
    fn from(other: Wind) -> Self {
        Self {
            yaw: other.yaw,
            pitch: other.pitch,
            roll: other.roll,
            velocity: other.velocity,
        }
    }
}
impl Default for WindBuilder {
    fn default() -> Self {
        Self {
            yaw: Angle::Radians(0.0),
            pitch: Angle::Radians(0.0),
            roll: Angle::Radians(0.0),
            velocity: Velocity::Mph(0.0),
        }
    }
}

impl WindAdjuster for SimulationBuilder {
    fn set_wind_speed(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.wind.velocity = Velocity::Mph(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn set_wind_angle(mut self, value: Numeric) -> Result<Self> {
        let (min, max) = (-360.0, 360.0);
        if value >= min && value <= max {
            self.wind.yaw = Angle::Degrees(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::OutOfRange(min, max)))
        }
    }
}

impl Wind {
    // This vector indicates direction of wind flow, not source of wind
    // so rotate by PI (adding or subtraction should have the same affect)
    // Negative indicates 90 degree wind is from east=>west
    // 0 degree wind is from north=>south (conventional)
    //        (0)
    //         ^
    //         |
    // (+90) <---> (-90)
    //         |
    //         v
    //       (180)
    //
    //  {after rotation(+ PI)}
    //
    //       (180)
    //         ^
    //         |
    // (-90) <---> (+90)
    //         |
    //         v
    //        (0)
    //
    //  {after negation(-)}
    //
    //       (180)
    //         ^
    //         |
    // (+90) <---> (-90)
    //         |
    //         v
    //        (0)
    fn yaw(&self) -> Angle {
        -self.yaw + Angle::Radians(PI)
    }
    fn pitch(&self) -> Angle {
        self.pitch
    }
    fn roll(&self) -> Angle {
        self.roll
    }
    pub(crate) fn velocity(&self) -> Vector3<Numeric> {
        self.velocity
            .to_mps()
            .to_num()
            .mul(Vector3::x())
            .pivot_y(self.yaw())
            .pivot_z(self.pitch())
            .pivot_x(self.roll())
    }
}
