use crate::model::builder::{FlagsBuilder, SimulationBuilder};
use crate::model::core::Flags;
use crate::util::*;

impl Default for Flags {
    fn default() -> Self {
        Self {
            use_coriolis: true,
            use_drag: true,
            use_gravity: true,
        }
    }
}

impl FlagsBuilder for SimulationBuilder {
    fn use_coriolis(mut self, value: bool) -> Result<Self> {
        self.flags.use_coriolis = value;
        Ok(self)
    }
    fn use_drag(mut self, value: bool) -> Result<Self> {
        self.flags.use_drag = value;
        Ok(self)
    }
    fn use_gravity(mut self, value: bool) -> Result<Self> {
        self.flags.use_gravity = value;
        Ok(self)
    }
}
