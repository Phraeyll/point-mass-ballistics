use crate::{
    consts::PI,
    error::{Error, Result},
    units::{
        pound, square_inch,
        typenum::P2,
        typenum::{N2, P1, Z0},
        Area, Length, Mass, MyQuantity, Ratio, Velocity, ISQ,
    },
    Numeric, NumericMap,
};

use std::{
    sync::OnceLock,
    ops::{Deref, DerefMut},
};

pub type SectionalDensity = MyQuantity<ISQ<N2, P1, Z0, Z0, Z0, Z0, Z0>>;

pub trait Projectile {
    fn area(&self) -> Area {
        PI * self.radius().powi(P2::new())
    }
    fn i(&self) -> Ratio {
        self.sd() / self.bc()
    }

    fn mass(&self) -> Mass;
    fn radius(&self) -> Length;
    fn velocity(&self) -> Velocity;
    fn bc(&self) -> SectionalDensity;
    fn sd(&self) -> SectionalDensity;
    fn cd(&self, x: Numeric) -> Result<Numeric>;
}

pub struct ProjectileImpl {
    pub caliber: Length,
    pub weight: Mass,
    pub bc: Numeric,
    pub velocity: Velocity,
}

macro_rules! drag_tables {
    ($($struct:ident => $module:ident,)+) => {
        drag_tables!{$($struct => $module),+}
    };
    ($($struct:ident => $module:ident),*) => {
        $(
            mod $module;
            pub struct $struct(ProjectileImpl);
            impl From<ProjectileImpl> for $struct {
                fn from(other: ProjectileImpl) -> Self {
                    Self(other)
                }
            }
            impl Deref for $struct {
                type Target = ProjectileImpl;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl DerefMut for $struct
            {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
            impl Projectile for $struct {
                fn velocity(&self) -> Velocity {
                    self.0.velocity
                }
                fn mass(&self) -> Mass {
                    self.0.weight
                }
                fn radius(&self) -> Length {
                    self.0.caliber / 2.0
                }
                fn bc(&self) -> SectionalDensity {
                    Mass::new::<pound>(self.0.bc) / Area::new::<square_inch>(1.0)
                }
                fn sd(&self) -> SectionalDensity {
                    self.0.weight / self.0.caliber.powi(P2::new())
                }
                // TABLE is a map of "mach speed" to "coefficients of drag", {x => y}
                // This funtions returns linear approximation of coefficient, for a given mach speed
                // When x is present in the map, interpolation is equivalent to TABLE.get_value(x)
                fn cd(&self, x: Numeric) -> Result<Numeric> {
                    // TODO: Does not work if x exists in map as smallest key, ..x excludes it, so first step is None
                    static TABLE: OnceLock<NumericMap> = OnceLock::new();
                    let table = TABLE.get_or_init($module::table);
                    table.range(..x).rev()     // First = None if smallest key >= x, else Some((x0, &y0)) where x0 greatest key <  x
                        .zip(table.range(x..)) // First = None if greatest key <  x, else Some((x1, &y1)) where x1 smallest key >= x
                        .map(|((x0, &y0), (x1, &y1))| y0 + (x - x0) * ((y1 - y0) / (x1 - x0))) // Linear interpolation when x0 and x1 both exist
                        .next()
                        .ok_or(Error::VelocityLookup(x)) // None => Err: x is outside of key range: this function does not extrapolate
                }
            }
        )*
    };
}

drag_tables! {
    G1 => g1,
    G2 => g2,
    G5 => g5,
    G6 => g6,
    G7 => g7,
    G8 => g8,
    GI => gi,
    GS => gs,
}
