use nalgebra::Vector3;

use crate::model::core::{Bc, BcBuilder, BcKind, ProjectileAdjuster, Scope, SimulationBuilder};
use crate::util::*;

use std::ops::Mul;

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

impl Projectile {
    // Radius of projectile cross section in meters
    fn radius(&self) -> Numeric {
        self.caliber.to_meters().to_num() / 2.0
    }
    // Area of projectile in meters, used during drag force calculation
    pub(crate) fn area(&self) -> Numeric {
        PI * self.radius().powf(2.0)
    }
    // Mass of projectile in kgs, used during acceleration calculation in simulation iteration
    pub(crate) fn mass(&self) -> Numeric {
        self.weight.to_kgs().into()
    }
    // Sectional density of projectile, defined terms of lbs and inches, yet dimensionless
    fn sd(&self) -> Numeric {
        self.weight.to_lbs().to_num() / self.caliber.to_inches().to_num().powf(2.0)
    }
    // Form factor of projectile, calculated fro Ballistic Coefficient and Sectional Density (sd)
    pub(crate) fn i(&self) -> Numeric {
        self.sd() / self.bc.value()
    }
    pub(crate) fn velocity(&self, scope: &Scope) -> Vector3<Numeric> {
        self.velocity
            .to_mps()
            .to_num()
            .mul(Vector3::x())
            .pivot_y(scope.yaw())
            .pivot_z(scope.pitch())
    }
}
