use crate::{
    model::core::{Bc, BcBuilder, BcKind, ProjectileAdjuster, SimulationBuilder},
    util::*,
};

#[derive(Debug)]
pub struct Projectile {
    pub(crate) caliber: Length,    // Caliber (inches)
    pub(crate) weight: WeightMass, // Weight (grains)
    pub(crate) bc: Bc,             // Ballistic Coefficient
    pub(crate) velocity: Velocity, // Initial velocity (ft/s)
}
#[derive(Debug)]
pub struct ProjectileBuilder {
    pub caliber: Length,    // Caliber (inches)
    pub weight: WeightMass, // Weight (grains)
    pub bc: BcBuilder,      // Ballistic Coefficient
    pub velocity: Velocity, // Initial velocity (ft/s)
}
impl From<ProjectileBuilder> for Projectile {
    fn from(other: ProjectileBuilder) -> Self {
        Self {
            caliber: other.caliber,
            weight: other.weight,
            bc: Bc::from(other.bc),
            velocity: other.velocity,
        }
    }
}
impl From<Projectile> for ProjectileBuilder {
    fn from(other: Projectile) -> Self {
        Self {
            caliber: other.caliber,
            weight: other.weight,
            bc: BcBuilder::from(other.bc),
            velocity: other.velocity,
        }
    }
}
impl Default for ProjectileBuilder {
    fn default() -> Self {
        Self {
            caliber: Length::Inches(0.264),
            weight: WeightMass::Grains(140.0),
            bc: BcBuilder::default(),
            velocity: Velocity::Fps(2710.0),
        }
    }
}
impl ProjectileAdjuster for SimulationBuilder {
    fn set_caliber(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.projectile.caliber = Length::Inches(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
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
    fn set_bc(mut self, value: Numeric, kind: BcKind) -> Result<Self> {
        if value.is_sign_positive() {
            self.projectile.bc = BcBuilder::new(value, kind);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
}
