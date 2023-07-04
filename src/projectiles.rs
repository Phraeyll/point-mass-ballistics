use crate::{
    consts::PI,
    error::{Error, Result},
    units::{
        pound, square_inch,
        typenum::P2,
        typenum::{N2, P1, Z0},
        Area, Length, Mass, MyQuantity, Ratio, Velocity, ISQ,
    },
    Numeric,
};

use std::ops::{Deref, DerefMut};

pub type SectionalDensity = MyQuantity<ISQ<N2, P1, Z0, Z0, Z0, Z0, Z0>>;

pub struct Table<const N: usize> {
    x: [Numeric; N],
    y: [Numeric; N],
}

impl<const N: usize> Table<N> {
    pub const fn new(x: [Numeric; N], y: [Numeric; N]) -> Self {
        Self { x, y }
    }
    pub fn search(&self, x: Numeric) -> Option<(Numeric, Numeric, Numeric, Numeric)> {
        let mut iter = self.x.into_iter();
        let mut index = 0;
        loop {
            if let Some(n) = iter.next() {
                if n > x {
                    if index > 0 {
                        break Some((
                            self.x[index - 1],
                            self.y[index - 1],
                            self.x[index],
                            self.y[index],
                        ));
                    }
                    break None;
                }
            } else {
                break None;
            }
            index += 1;
        }
    }
    pub fn binary_search(&self, x: Numeric) -> Option<(Numeric, Numeric, Numeric, Numeric)> {
        if self.x.is_empty() {
            return None;
        }

        let mut low = 0;
        let mut high = self.x.len() - 1;
        while low <= high {
            let index = (high + low) / 2;
            if let Some(&current) = self.x.get(index) {
                if current > x {
                    if index == 0 {
                        return None;
                    }
                    high = index - 1
                }
                if current < x {
                    low = index + 1
                }
            }
        }
        if low < self.x.len() {
            Some((self.x[high], self.y[high], self.x[low], self.y[low]))
        } else {
            None
        }
    }
}

macro_rules! count {
    ($($t:tt,)+) => {
        count!($($t),+)
    };
    ($($t:tt),*) => {
        <[()]>::len(&[$(subst!($t, ())),*])
    };
}

macro_rules! subst {
    ($t:tt, $e:expr) => {
        $e
    };
}

macro_rules! table {
    ( $($x:expr => $y:expr,)+ ) => {
        table![$($x => $y),+];
    };
    ( $($x:expr => $y:expr),* ) => {
        const SIZE: usize = count!($($x,)*);
        pub const TABLE: $crate::projectiles::Table<SIZE> = $crate::projectiles::Table::new([$($x,)*], [$($y,)*]);
    };
}
pub(crate) use table;

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
                    // None => Err: x is outside of key range: this function does not extrapolate
                    let (x0, y0, x1, y1) = $module::TABLE.binary_search(x).ok_or(Error::VelocityLookup(x))?;

                    // Linear interpolation when x0 and x1 both exist
                    Ok(y0 + (x - x0) * ((y1 - y0) / (x1 - x0)))
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
