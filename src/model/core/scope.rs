use crate::{
    model::core::{ScopeAdjuster, SimulationBuilder},
    util::*,
};

#[derive(Debug)]
pub struct Scope {
    pub(crate) yaw: Angle,
    pub(crate) pitch: Angle,
    pub(crate) roll: Angle,    // Scope Roll (Cant) (Degrees)
    pub(crate) height: Length, // Scope Height (inches)
    pub(crate) offset: Length, // Scope Offset Windage (left/right boreline) (inches)
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
