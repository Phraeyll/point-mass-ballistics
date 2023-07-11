use crate::{
    error::{Error, Result},
    units::Ratio,
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

const fn len<const N: usize>(_: [(); N]) -> usize {
    N
}

macro_rules! count {
    ($($t:tt,)+) => {
        count!($($t),+)
    };
    ($($t:tt),*) => {
        len([$(void!($t)),*])
    };
}
use count;

macro_rules! void {
    ($t:tt) => {
        ()
    };
}
use void;

macro_rules! table {
    ( $($x:expr => $y:expr,)+ ) => {
        table![$($x => $y),+];
    };
    ( $($x:expr => $y:expr),* ) => {
        use super::*;
        use $crate::{
            consts::FRAC_PI_8,
            error::Result,
            physics::DragFunction,
            units::Ratio,
            Numeric,
            OPTIMIZE_DRAG_TABLE,
        };

        pub struct Drag;

        impl Drag {
            pub const TABLE: Table<{count!($($x,)*)}> = Table {
                x: [
                    $(
                        $x
                    ,)
                *],
                y: [
                    $(
                        if OPTIMIZE_DRAG_TABLE { -($y * FRAC_PI_8) } else { $y }
                    ,)*
                ],
            };
        }

        impl DragFunction for Drag {
            // TABLE is a effictely a map of "mach speed" to "drag coefficients", {x => y}
            // This funtions returns linear approximation of drag coefficient, for a given mach speed
            fn cd(x: Ratio) -> Result<Numeric> {
                Self::TABLE.cd(x)
            }
        }
    };
}
use table;

pub struct Table<const N: usize> {
    x: [Numeric; N],
    y: [Numeric; N],
}

impl<const N: usize> Table<N> {
    pub fn cd(&self, x: Ratio) -> Result<Numeric> {
        let x = x.value;
        // Find values in table to interpolate
        let i = self.binary_search(x)?;
        let j = i + 1;
        let (x0, y0, x1, y1) = (self.x[i], self.y[i], self.x[j], self.y[j]);

        // Linear interpolation
        Ok(y0 + (x - x0) * ((y1 - y0) / (x1 - x0)))
    }

    pub fn linear_search(&self, x: Numeric) -> Result<usize> {
        let mut iter = self.x.into_iter().enumerate();
        loop {
            if let Some((i, n)) = iter.next() {
                if n > x {
                    break Ok(i - 1);
                }
            } else {
                break Err(Error::Mach(x));
            }
        }
    }

    pub fn binary_search(&self, x: Numeric) -> Result<usize> {
        let mut low = 0;
        let mut high = N - 1;
        while low <= high {
            let index = (high + low) / 2;
            let x0 = self.x[index];
            if x0 > x {
                high = index - 1;
            } else if x0 < x {
                low = index + 1;
            } else {
                high = index;
                low = index;
                break;
            }
        }
        if low < N {
            Ok(high)
        } else {
            Err(Error::Mach(x))
        }
    }
}
