use crate::model::core::{FlagsBuilder, SimulationBuilder};
use crate::util::*;

#[derive(Debug)]
pub struct Flags {
    pub(crate) use_coriolis: bool, // Whether or not to calculate coriolis/eotvos effect
    pub(crate) use_drag: bool,     // Whether or not to calculate drag
    pub(crate) use_gravity: bool,  // Whether or not to calculate gravity
}

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
