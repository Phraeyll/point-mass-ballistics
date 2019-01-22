use crate::util::*;
use crate::model::core::Angles;

impl Default for Angles {
    fn default() -> Self {
        Self {
            pitch: Angle::Radians(0.0),
            yaw: Angle::Radians(0.0),
        }
    }
}

pub trait MutateAngles {
    fn new() -> Self;
    fn set_pitch(self, value: Numeric) -> Self;
    fn set_yaw(self, value: Numeric) -> Self;
}
impl MutateAngles for Angles {
    fn new() -> Self {
        Self::default()
    }
    fn set_pitch(mut self, value: Numeric) -> Self {
        self.pitch = Angle::Minutes(value);
        self
    }
    fn set_yaw(mut self, value: Numeric) -> Self {
        self.yaw = Angle::Minutes(value);
        self
    }
}
