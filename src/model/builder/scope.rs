use crate::util::*;
use crate::model::core::Scope;
use crate::model::builder::{SimulationBuilder, ScopeBuilder};

impl Default for Scope {
    fn default() -> Self {
        Self {
            height: Length::Inches(1.5),
            offset: Length::Inches(0.0),
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
}
