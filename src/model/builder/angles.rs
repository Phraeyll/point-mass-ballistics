use crate::util::*;
use crate::model::core::Angles;
use crate::model::builder::{SimulationBuilder, AnglesBuilder};

impl Default for Angles {
    fn default() -> Self {
        Self {
            pitch: Angle::Radians(0.0),
            yaw: Angle::Radians(0.0),
        }
    }
}

impl AnglesBuilder for SimulationBuilder {
    fn set_pitch(mut self, value: Numeric) -> Self {
        self.angles.pitch = Angle::Minutes(value);
        self
    }
    fn set_yaw(mut self, value: Numeric) -> Self {
        self.angles.yaw = Angle::Minutes(value);
        self
    }
}
