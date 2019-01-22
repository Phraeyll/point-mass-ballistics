use super::dragtables::*;
use crate::model::core::{Bc, BcKind, BcKind::*, Projectile};
use crate::model::builder::{SimulationBuilder, ProjectileBuilder};
use crate::util::*;

impl Default for Bc {
    fn default() -> Self {
        // Arbitrary data - intended to be set by with method above at initialization point
        Self {
            value: 0.0,
            kind: G1,
            table: float_map![],
        }
    }
}

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

pub trait BcBuilder {
    fn with(value: Numeric, kind: BcKind) -> Result<Self>
    where
        Self: Sized;
}
impl BcBuilder for Bc {
    fn with(value: Numeric, kind: BcKind) -> Result<Self> {
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
    fn set_bc(mut self, value: Bc) -> Self {
        self.projectile.bc = value;
        self
    }
}
