pub use self::BcKind::*;
use crate::{
    model::core::{dragtables::*, ProjectileAdjuster, SimulationBuilder},
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
pub struct Bc {
    pub(crate) value: Numeric,
    pub(crate) kind: BcKind,
    pub(crate) table: FloatMap<Numeric>,
}
#[derive(Debug, Copy, Clone)]
pub enum BcKind {
    G1,
    G2,
    G5,
    G6,
    G7,
    G8,
    GI,
    GS,
    Null,
}
impl Bc {
    pub fn value(&self) -> Numeric {
        self.value
    }
    pub fn table(&self) -> &FloatMap<Numeric> {
        &self.table
    }
    pub fn kind(&self) -> BcKind {
        self.kind
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
            self.projectile.bc = Bc {
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
                    Null => float_map![],
                },
            };
            Ok(self)
        } else {
            Err(Error::new(ErrorKind::PositiveExpected(value)))
        }
    }
}
