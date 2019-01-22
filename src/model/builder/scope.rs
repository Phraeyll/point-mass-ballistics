use crate::util::*;
use crate::model::core::Scope;

impl Default for Scope {
    fn default() -> Self {
        Self {
            height: Length::Inches(1.5),
            offset: Length::Inches(0.0),
        }
    }
}

pub trait MutateScope {
    fn new() -> Self;
    fn set_height(self, value: Numeric) -> Self;
    fn set_offset(self, value: Numeric) -> Self;
}
impl MutateScope for Scope {
    fn new() -> Self {
        Self::default()
    }
    fn set_height(mut self, value: Numeric) -> Self {
        self.height = Length::Inches(value);
        self
    }
    fn set_offset(mut self, value: Numeric) -> Self {
        self.offset = Length::Inches(value);
        self
    }
}
