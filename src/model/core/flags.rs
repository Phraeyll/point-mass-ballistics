use crate::model::core::{FlagsAdjuster, SimulationBuilder};
use crate::util::*;

#[derive(Debug)]
pub struct Flags {
    pub(crate) coriolis: bool, // Whether or not to calculate coriolis/eotvos effect
    pub(crate) drag: bool,     // Whether or not to calculate drag
    pub(crate) gravity: bool,  // Whether or not to calculate gravity
}
#[derive(Debug)]
pub struct FlagsBuilder {
    pub coriolis: bool, // Whether or not to calculate coriolis/eotvos effect
    pub drag: bool,     // Whether or not to calculate drag
    pub gravity: bool,  // Whether or not to calculate gravity
}
impl From<FlagsBuilder> for Flags {
    fn from(other: FlagsBuilder) -> Self {
        Self {
            coriolis: other.coriolis,
            drag: other.drag,
            gravity: other.gravity,
        }
    }
}
impl From<Flags> for FlagsBuilder {
    fn from(other: Flags) -> Self {
        Self {
            coriolis: other.coriolis,
            drag: other.drag,
            gravity: other.gravity,
        }
    }
}
impl Default for FlagsBuilder {
    fn default() -> Self {
        Self {
            coriolis: true,
            drag: true,
            gravity: true,
        }
    }
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

impl Flags {
    pub(crate) fn coriolis(&self) -> bool {
        self.coriolis
    }
    pub(crate) fn drag(&self) -> bool {
        self.drag
    }
    pub(crate) fn gravity(&self) -> bool {
        self.gravity
    }
}
