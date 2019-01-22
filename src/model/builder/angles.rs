use crate::util::*;

#[derive(Debug)]
pub struct Angles {
    pub(crate) pitch: Angle,
    pub(crate) yaw: Angle,
    pub(crate) roll: Angle,
}
impl Default for Angles {
    fn default() -> Self {
        Self {
            pitch: Angle::Radians(0.0),
            yaw: Angle::Radians(0.0),
            roll: Angle::Radians(0.0),
        }
    }
}

pub trait MutateAngles {
    fn new() -> Self;
    fn with_pitch(self, value: Numeric) -> Self;
    fn with_yaw(self, value: Numeric) -> Self;
    fn with_roll(self, value: Numeric) -> Self;
}
impl MutateAngles for Angles {
    fn new() -> Self {
        Self::default()
    }
    fn with_pitch(mut self, value: Numeric) -> Self {
        self.pitch = Angle::Minutes(value);
        self
    }
    fn with_yaw(mut self, value: Numeric) -> Self {
        self.yaw = Angle::Minutes(value);
        self
    }
    fn with_roll(mut self, value: Numeric) -> Self {
        self.roll = Angle::Minutes(value);
        self
    }
}
