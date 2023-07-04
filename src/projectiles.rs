use crate::{
    consts::PI,
    error::Result,
    units::{typenum::P2, Area, ArealMassDensity, Length, Mass, Ratio, Velocity},
    Numeric,
};

pub mod g1;
pub mod g2;
pub mod g5;
pub mod g6;
pub mod g7;
pub mod g8;
pub mod gi;
pub mod gs;

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
    fn bc(&self) -> ArealMassDensity;
    fn sd(&self) -> ArealMassDensity;
    fn cd(&self, x: Numeric) -> Result<Numeric>;
}

pub struct ProjectileImpl {
    pub caliber: Length,
    pub weight: Mass,
    pub bc: Numeric,
    pub velocity: Velocity,
}

const fn len<const N: usize, T>(_: &[T; N]) -> usize {
    N
}
macro_rules! count {
    ($($t:tt,)+) => {
        count!($($t),+)
    };
    ($($t:tt),*) => {
        len(&[$(subst!($t, ())),*])
    };
}
pub(crate) use count;

macro_rules! subst {
    ($t:tt, $e:expr) => {
        $e
    };
}
pub(crate) use subst;

macro_rules! table {
    ( $($x:expr => $y:expr,)+ ) => {
        table![$($x => $y),+];
    };
    ( $($x:expr => $y:expr),* ) => {
        use super::*;
        use $crate::units::{
            pound, square_inch, typenum::P2, Area, ArealMassDensity, Length, Mass, Velocity,
        };

        pub struct P(ProjectileImpl);

        impl P {
            pub const TABLE: Table<{count!($($x,)*)}> = Table::new(
                [$($x,)*],
                [$($y,)*],
            );
        }
        impl From<ProjectileImpl> for P {
            fn from(other: ProjectileImpl) -> Self {
                Self(other)
            }
        }
        impl std::ops::Deref for P {
            type Target = ProjectileImpl;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl std::ops::DerefMut for P {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl Projectile for P {
            fn velocity(&self) -> Velocity {
                self.0.velocity
            }
            fn mass(&self) -> Mass {
                self.0.weight
            }
            fn radius(&self) -> Length {
                self.0.caliber / 2.0
            }
            fn bc(&self) -> ArealMassDensity {
                let mass = Mass::new::<pound>(self.0.bc);
                let area = Area::new::<square_inch>(1.0);
                mass / area
            }
            fn sd(&self) -> ArealMassDensity {
                self.0.weight / self.0.caliber.powi(P2::new())
            }
            // TABLE is a map of "mach speed" to "coefficients of drag", {x => y}
            // This funtions returns linear approximation of coefficient, for a given mach speed
            // When x is present in the map, interpolation is equivalent to TABLE.get_value(x)
            fn cd(&self, x: $crate::Numeric) -> $crate::error::Result<$crate::Numeric> {
                // None => Err: x is outside of key range: this function does not extrapolate
                let (x0, y0, x1, y1) = Self::TABLE
                    .binary_search(x)
                    .ok_or($crate::error::Error::VelocityLookup(x))?;

                // Linear interpolation when x0 and x1 both exist
                Ok(y0 + (x - x0) * ((y1 - y0) / (x1 - x0)))
            }
        }
    };
}
pub(crate) use table;

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
