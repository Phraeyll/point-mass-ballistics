use super::dragtables::*;
use crate::model::core::{Bc, BcKind, Projectile};
use crate::util::*;

pub trait BcBuilder {
    fn new(value: Numeric, kind: BcKind) -> Result<Self>
    where
        Self: Sized;
}
impl BcBuilder for Bc {
    fn new(value: Numeric, kind: BcKind) -> Result<Self> {
        if value.is_sign_positive() {
            Ok(Self {
                value,
                kind,
                table: match kind {
                    BcKind::G1 => g1::init(),
                    BcKind::G2 => g2::init(),
                    BcKind::G5 => g5::init(),
                    BcKind::G6 => g6::init(),
                    BcKind::G7 => g7::init(),
                    BcKind::G8 => g8::init(),
                    BcKind::GI => gi::init(),
                    BcKind::GS => gs::init(),
                },
            })
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            weight: WeightMass::Grains(140.0),
            caliber: Length::Inches(0.264),
            bc: Bc::new(0.305, BcKind::G7).expect("how"),
            velocity: Velocity::Fps(2710.0),
        }
    }
}

pub trait MutateProjectile {
    fn new() -> Self;
    fn set_velocity(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;

    fn set_grains(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;

    fn set_caliber(self, value: Numeric) -> Result<Self>
    where
        Self: Sized;

    fn set_bc(self, value: Bc) -> Self;
}
impl MutateProjectile for Projectile {
    fn new() -> Self {
        Self::default()
    }
    fn set_velocity(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.velocity = Velocity::Fps(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn set_grains(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.weight = WeightMass::Grains(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn set_caliber(mut self, value: Numeric) -> Result<Self> {
        if value.is_sign_positive() {
            self.caliber = Length::Inches(value);
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
    fn set_bc(mut self, value: Bc) -> Self {
        self.bc = value;
        self
    }
}
