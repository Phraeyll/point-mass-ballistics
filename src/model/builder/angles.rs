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
    fn set_pitch(mut self, value: Numeric) -> Result<Self> {
        self.angles.pitch = Angle::Minutes(value);
        Ok(self)
    }
    fn set_yaw(mut self, value: Numeric) -> Result<Self> {
        self.angles.yaw = Angle::Minutes(value);
        Ok(self)
    }
    fn increment_pitch(mut self, value: Numeric) -> Result<Self> {
        self.angles.pitch = Angle::Minutes(
            self.angles.pitch.to_minutes().to_num() +
            value
        );
        Ok(self)
    }
    fn increment_yaw(mut self, value: Numeric) -> Result<Self> {
        self.angles.yaw = Angle::Minutes(
            self.angles.yaw.to_minutes().to_num() +
            value
        );
        Ok(self)
    }
}
