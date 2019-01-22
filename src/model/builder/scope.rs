use crate::util::*;

#[derive(Debug)]
pub struct Scope {
    pub(crate) height: Length, // Scope Height (inches)
    pub(crate) offset: Length, // Scope Offset Windage (left/right boreline) (inches)
}
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
    fn with_height(self, value: Numeric) -> Self;
    fn with_offset(self, value: Numeric) -> Self;
}
impl MutateScope for Scope {
    fn new() -> Self {
        Self::default()
    }
    fn with_height(mut self, value: Numeric) -> Self {
        self.height = Length::Inches(value);
        self
    }
    fn with_offset(mut self, value: Numeric) -> Self {
        self.offset = Length::Inches(value);
        self
    }
}
