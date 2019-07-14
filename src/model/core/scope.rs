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
    fn set_scope_height(mut self, value: Numeric) -> Self {
        self.builder.scope.height = Length::Inches(value);
        self
    }
    fn set_scope_offset(mut self, value: Numeric) -> Self {
        self.builder.scope.offset = Length::Inches(value);
        self
    }
    fn set_scope_pitch(mut self, value: Numeric) -> Self {
        self.builder.scope.pitch = Angle::Minutes(value);
        self
    }
    fn set_scope_yaw(mut self, value: Numeric) -> Self {
        self.builder.scope.yaw = Angle::Minutes(value);
        self
    }
    fn set_scope_roll(mut self, value: Numeric) -> Self {
        self.builder.scope.roll = Angle::Degrees(value);
        self
    }
}
