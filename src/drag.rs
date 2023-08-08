use crate::{
    error::{Error, Result},
    units::{ReciprocalLength, Velocity},
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
            simulation::Simulation,
            units::{ReciprocalLength, Velocity},
        };

        use std::sync::OnceLock;

        pub const SIZE: usize = count!($($x,)*);
        pub static TABLE: OnceLock<Table<SIZE>> = OnceLock::new();
        pub struct Drag;
        impl DragFunction for Drag {
            fn init(simulation: &Simulation<Self>) {
                let _ = TABLE.set(Table {
                    x: [
                        $(
                            $x * simulation.atmosphere.sound_velocity()
                        ),*
                    ],
                    y: [
                        $(
                            -($y * FRAC_PI_8) * simulation.atmosphere.rho() / simulation.projectile.bc()
                        ),*
                    ],
                });
            }
            fn cd(velocity: Velocity) -> Result<ReciprocalLength> {
                TABLE.get().unwrap().lerp(velocity)
            }
        }
    };
}
use table;

pub struct Table<const N: usize> {
    x: [Velocity; N],
    y: [ReciprocalLength; N],
}

impl<const N: usize> Table<N> {
    pub fn lerp(&self, x: Velocity) -> Result<ReciprocalLength> {
        // Find values in table to interpolate
        let (i, j) = self.binary_search(x)?;
        let (x0, y0) = (self.x[i], self.y[i]);
        let (x1, y1) = (self.x[j], self.y[j]);

        // Linear interpolation
        let y = y0 + (x - x0) * ((y1 - y0) / (x1 - x0));
        // let y = (y0 * (x1 - x0)) / (x1 - x0) + (y1 * (x - x0) - y0 * (x - x0)) / (x1 - x0);
        // let y = (y1 * x - y1 * x0 - y0 * x + y0 * x0 + y0 * x1 - y0 * x0) / (x1 - x0);
        // let y = (y0 * (x1 - x) + y1 * (x - x0)) / (x1 - x0);
        Ok(y)
    }

    pub fn linear_search(&self, x: Velocity) -> Result<(usize, usize)> {
        let mut iter = self.x.into_iter().enumerate();
        loop {
            if let Some((i, n)) = iter.next() {
                if n > x {
                    break Ok((i - 1, i));
                }
            } else {
                break Err(Error::Velocity(x));
            }
        }
    }

    pub fn binary_search(&self, x: Velocity) -> Result<(usize, usize)> {
        let mut low = 0;
        let mut high = N - 1;
        while low <= high {
            let index = (high + low) / 2;
            let n = self.x[index];
            if n > x {
                high = index - 1;
            } else if n < x {
                low = index + 1;
            } else {
                high = index;
                low = index;
                break;
            }
        }
        if low < N {
            Ok((high, low))
        } else {
            Err(Error::Velocity(x))
        }
    }
}
