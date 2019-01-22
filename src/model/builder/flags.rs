use crate::model::core::Flags;

impl Default for Flags {
    fn default() -> Self {
        Self {
            use_coriolis: true,
            use_drag: true,
            use_gravity: true,
        }
    }
}

pub trait MutateFlags {
    fn new() -> Self;
    fn enable_coriolis(self, value: bool) -> Self;
    fn enable_drag(self, value: bool) -> Self;
    fn enable_gravity(self, value: bool) -> Self;
}
impl MutateFlags for Flags {
    fn new() -> Self {
        Self::default()
    }
    fn enable_coriolis(mut self, value: bool) -> Self {
        self.use_coriolis = value;
        self
    }
    fn enable_drag(mut self, value: bool) -> Self {
        self.use_drag = value;
        self
    }
    fn enable_gravity(mut self, value: bool) -> Self {
        self.use_gravity = value;
        self
    }
}
