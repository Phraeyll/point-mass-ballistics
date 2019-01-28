use nalgebra::Vector3;

use crate::model::core::{ScopeAdjuster, SimulationBuilder};
use crate::util::*;

#[derive(Debug)]
pub struct Scope {
    pub(crate) yaw: Angle,
    pub(crate) pitch: Angle,
    pub(crate) roll: Angle,    // Scope Roll (Cant) (Degrees)
    pub(crate) height: Length, // Scope Height (inches)
    pub(crate) offset: Length, // Scope Offset Windage (left/right boreline) (inches)
}
#[derive(Debug)]
pub struct ScopeBuilder {
    pub yaw: Angle,
    pub pitch: Angle,
    pub roll: Angle,    // Scope Roll (Cant) (Degrees)
    pub height: Length, // Scope Height (inches)
    pub offset: Length, // Scope Offset Windage (left/right boreline) (inches)
}
impl From<ScopeBuilder> for Scope {
    fn from(other: ScopeBuilder) -> Self {
        Self {
            yaw: other.yaw,
            pitch: other.pitch,
            roll: other.roll,
            height: other.height,
            offset: other.offset,
        }
    }
}
impl From<Scope> for ScopeBuilder {
    fn from(other: Scope) -> Self {
        Self {
            yaw: other.yaw,
            pitch: other.pitch,
            roll: other.roll,
            height: other.height,
            offset: other.offset,
        }
    }
}
impl Default for ScopeBuilder {
    fn default() -> Self {
        Self {
            yaw: Angle::Radians(0.0),
            pitch: Angle::Radians(0.0),
            roll: Angle::Radians(0.0),
            height: Length::Inches(1.5),
            offset: Length::Inches(0.0),
        }
    }
}
impl ScopeAdjuster for SimulationBuilder {
    fn set_scope_height(mut self, value: Numeric) -> Result<Self> {
        self.scope.height = Length::Inches(value);
        Ok(self)
    }
    fn set_scope_offset(mut self, value: Numeric) -> Result<Self> {
        self.scope.offset = Length::Inches(value);
        Ok(self)
    }
    fn set_scope_pitch(mut self, value: Numeric) -> Result<Self> {
        self.scope.pitch = Angle::Minutes(value);
        Ok(self)
    }
    fn set_scope_yaw(mut self, value: Numeric) -> Result<Self> {
        self.scope.yaw = Angle::Minutes(value);
        Ok(self)
    }
    fn set_scope_roll(mut self, value: Numeric) -> Result<Self> {
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
    pub(crate) fn pitch(&self) -> Angle {
        self.pitch
    }
    pub(crate) fn yaw(&self) -> Angle {
        -self.yaw
    }
    pub(crate) fn roll(&self) -> Angle {
        -self.roll
    }
}