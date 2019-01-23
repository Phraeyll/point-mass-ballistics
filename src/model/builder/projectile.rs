use crate::model::builder::{ProjectileBuilder, SimulationBuilder};
use crate::model::core::{Bc, Projectile};
use crate::util::*;

impl Default for Projectile {
    fn default() -> Self {
        Self {
            weight: WeightMass::Grains(140.0),
            caliber: Length::Inches(0.264),
            bc: Bc::default(),
            velocity: Velocity::Fps(2710.0),
        }
    }
}

impl ProjectileBuilder for SimulationBuilder {
    fn set_velocity(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.projectile.velocity = Velocity::Fps(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn set_grains(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.projectile.weight = WeightMass::Grains(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn set_caliber(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.projectile.caliber = Length::Inches(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
}
