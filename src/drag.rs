use crate::{
    error::{Error, Result},
    Numeric,
};

pub use crate::physics::DragFunction;

pub mod g1;
pub mod g2;
pub mod g5;
pub mod g6;
pub mod g7;
pub mod g8;
pub mod gi;
pub mod gs;

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
        use $crate::{
            physics::DragFunction,
            Numeric,
            error::Result,
        };

        pub struct Drag;

        impl Drag {
            pub const TABLE: Table<{count!($($x,)*)}> = Table::new(
                [$($x,)*],
                [$($y,)*],
            );
        }

        impl DragFunction for Drag {
            // TABLE is a effictely a map of "mach speed" to "drag coefficients", {x => y}
            // This funtions returns linear approximation of drag coefficient, for a given mach speed
            fn cd(x: Numeric) -> Result<Numeric> {
                // Find values in table to interpolate
                let (x0, y0, x1, y1) = Self::TABLE.binary_search(x)?;

                // Linear interpolation
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
