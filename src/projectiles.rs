use crate::{
    consts::PI,
    error::{Error, Result},
    units::{
        pound, square_inch, typenum::P2, Area, ArealMassDensity, Length, Mass, Ratio, Velocity,
    },
    Numeric,
};

use std::marker::PhantomData;

pub mod g1;
pub mod g2;
pub mod g5;
pub mod g6;
pub mod g7;
pub mod g8;
pub mod gi;
pub mod gs;

pub trait DragFunction {
    fn cd(mach: Numeric) -> Result<Numeric>;
}

#[derive(Debug)]
pub struct Projectile<D: DragFunction> {
    pub caliber: Length,
    pub weight: Mass,
    pub bc: Numeric,
    pub velocity: Velocity,
    pub _marker: PhantomData<D>,
}

impl<D> Projectile<D>
where
    D: DragFunction,
{
    pub fn area(&self) -> Area {
        PI * self.radius().powi(P2::new())
    }

    pub fn i(&self) -> Ratio {
        self.sd() / self.bc()
    }

    pub fn radius(&self) -> Length {
        self.caliber / 2.0
    }

    pub fn bc(&self) -> ArealMassDensity {
        let mass = Mass::new::<pound>(self.bc);
        let area = Area::new::<square_inch>(1.0);
        mass / area
    }

    pub fn sd(&self) -> ArealMassDensity {
        self.weight / self.caliber.powi(P2::new())
    }

    pub fn cd(&self, x: Numeric) -> Result<Numeric> {
        D::cd(x)
    }
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

        pub struct Drag;

        impl Drag {
            pub const TABLE: Table<{count!($($x,)*)}> = Table::new(
                [$($x,)*],
                [$($y,)*],
            );
        }

        impl DragFunction for Drag {
            // TABLE is a map of "mach speed" to "coefficients of drag", {x => y}
            // This funtions returns linear approximation of coefficient, for a given mach speed
            // When x is present in the map, interpolation is equivalent to TABLE.get_value(x)
            fn cd(x: $crate::Numeric) -> $crate::error::Result<$crate::Numeric> {
                // None => Err: x is outside of key range: this function does not extrapolate
                let (x0, y0, x1, y1) = Self::TABLE.binary_search(x)?;
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

    pub fn linear_search(&self, x: Numeric) -> Result<(Numeric, Numeric, Numeric, Numeric)> {
        let mut iter = self.x.into_iter().enumerate();
        loop {
            if let Some((i, n)) = iter.next() {
                if n > x {
                    break Ok((self.x[i - 1], self.y[i - 1], self.x[i], self.y[i]));
                }
            } else {
                break Err(Error::Mach(x));
            }
        }
    }

    pub fn binary_search(&self, x: Numeric) -> Result<(Numeric, Numeric, Numeric, Numeric)> {
        if self.x.is_empty() {
            unreachable!()
        }

        let mut low = 0;
        let mut high = self.x.len() - 1;
        while low <= high {
            let index = (high + low) / 2;
            if let Some(&current) = self.x.get(index) {
                if current == x {
                    high = index;
                    low = index;
                    break;
                }
                if current > x {
                    high = index - 1;
                }
                if current < x {
                    low = index + 1;
                }
            }
        }
        if low < self.x.len() {
            Ok((self.x[high], self.y[high], self.x[low], self.y[low]))
        } else {
            Err(Error::Mach(x))
        }
    }
}
