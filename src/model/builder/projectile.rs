pub use BallisticCoefficientKind::*;

use crate::util::*;
use crate::model::builder::dragtables::*;

#[derive(Debug, Copy, Clone)]
pub enum BallisticCoefficientKind {
    G1,
    G2,
    G5,
    G6,
    G7,
    G8,
    GI,
    GS,
}
#[derive(Debug)]
pub struct BallisticCoefficient {
    pub(crate) value: Numeric,
    pub(crate) kind: BallisticCoefficientKind,
    pub(crate) table: FloatMap<Numeric>,
}
pub trait BallisticCoefficientBuilder {
    fn new(value: Numeric, kind: BallisticCoefficientKind) -> Result<Self>
    where
        Self: Sized;
}
impl BallisticCoefficientBuilder for BallisticCoefficient {
    fn new(value: Numeric, kind: BallisticCoefficientKind) -> Result<Self> {
        if value.is_sign_positive() {
            Ok(Self {
                value,
                kind,
                table: match kind {
                    G1 => g1::init(),
                    G2 => g2::init(),
                    G5 => g5::init(),
                    G6 => g6::init(),
                    G7 => g7::init(),
                    G8 => g8::init(),
                    GI => gi::init(),
                    GS => gs::init(),
                },
            })
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
}

#[derive(Debug)]
pub struct Projectile {
    pub(crate) weight: WeightMass,       // Weight (grains)
    pub(crate) caliber: Length,          // Caliber (inches)
    pub(crate) bc: BallisticCoefficient, // Ballistic Coefficient
    pub(crate) velocity: Velocity,       // Initial velocity (ft/s)
}
impl Default for Projectile {
    fn default() -> Self {
        Self {
            weight: WeightMass::Grains(140.0),
            caliber: Length::Inches(0.264),
            bc: BallisticCoefficient::new(0.305, G7).expect("how"),
            velocity: Velocity::Fps(2710.0),
        }
    }
}

pub trait MutateProjectile {
    fn new() -> Self;
    fn with_velocity(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;

    fn with_grains(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;

    fn with_caliber(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;

    fn with_bc(self, value: BallisticCoefficient) -> Self;
}
impl MutateProjectile for Projectile {
    fn new() -> Self {
        Self::default()
    }
    fn with_velocity(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.velocity = Velocity::Fps(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn with_grains(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.weight = WeightMass::Grains(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn with_caliber(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.caliber = Length::Inches(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn with_bc(mut self, value: BallisticCoefficient) -> Self {
        self.bc = value;
        self
    }
}
