use nalgebra::Vector3;

use crate::model::core::{ScopeBuilder, SimulationBuilder};
use crate::util::*;

#[derive(Debug)]
pub struct Scope {
    pub(crate) height: Length, // Scope Height (inches)
    pub(crate) offset: Length, // Scope Offset Windage (left/right boreline) (inches)
    pub(crate) pitch: Angle,
    pub(crate) yaw: Angle,
    pub(crate) roll: Angle, // Scope Roll (Cant) (Degrees)
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            height: Length::Inches(1.5),
            offset: Length::Inches(0.0),
            pitch: Angle::Radians(0.0),
            yaw: Angle::Radians(0.0),
            roll: Angle::Radians(0.0),
        }
    }
}

impl ScopeBuilder for SimulationBuilder {
    fn set_height(mut self, value: Numeric) -> Result<Self> {
        self.scope.height = Length::Inches(value);
        Ok(self)
    }
    fn set_offset(mut self, value: Numeric) -> Result<Self> {
        self.scope.offset = Length::Inches(value);
        Ok(self)
    }
    fn set_pitch(mut self, value: Numeric) -> Result<Self> {
        self.scope.pitch = Angle::Minutes(value);
        Ok(self)
    }
    fn set_yaw(mut self, value: Numeric) -> Result<Self> {
        self.scope.yaw = Angle::Minutes(value);
        Ok(self)
    }
    fn set_roll(mut self, value: Numeric) -> Result<Self> {
        self.scope.roll = Angle::Degrees(value);
        Ok(self)
    }
}

impl Scope {
    pub(crate) fn position(&self) -> Vector3<Numeric> {
        Vector3::new(
            0.0,
            self.height.to_meters().to_num(),
            self.offset.to_meters().to_num(),
        )
    }
}
