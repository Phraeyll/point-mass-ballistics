use crate::{
    model::core::{FlagsAdjuster, SimulationBuilder},
    util::*,
};

#[derive(Debug)]
pub struct Flags {
    pub(crate) coriolis: bool, // Whether or not to calculate coriolis/eotvos effect
    pub(crate) drag: bool,     // Whether or not to calculate drag
    pub(crate) gravity: bool,  // Whether or not to calculate gravity
}
impl FlagsAdjuster for SimulationBuilder {
    fn use_coriolis(mut self, value: bool) -> Result<Self> {
        self.flags.coriolis = value;
        Ok(self)
    }
    fn use_drag(mut self, value: bool) -> Result<Self> {
        self.flags.drag = value;
        Ok(self)
    }
    fn use_gravity(mut self, value: bool) -> Result<Self> {
        self.flags.gravity = value;
        Ok(self)
    }
}
