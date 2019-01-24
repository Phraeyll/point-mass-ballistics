use nalgebra::Vector3;

use crate::model::core::{WindBuilder, SimulationBuilder};
use crate::util::*;

use std::ops::Mul;

#[derive(Debug)]
pub struct Wind {
    pub(crate) velocity: Velocity, // Wind Velocity (miles/hour)
    pub(crate) yaw: Angle,         // Wind Angle (degrees)
}

impl Default for Wind {
    fn default() -> Self {
        Self {
            velocity: Velocity::Mph(0.0),
            yaw: Angle::Radians(0.0),
        }
    }
}

impl WindBuilder for SimulationBuilder {
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
    pub(crate) fn corrected_yaw(&self) -> Angle {
        Angle::Radians(-self.yaw.to_radians().to_num() + PI)
    }
    pub(crate) fn velocity(&self) -> Vector3<Numeric> {
        self.velocity
            .to_mps()
            .to_num()
            .mul(Vector3::x())
            .pivot_y(self.corrected_yaw())
    }
}
