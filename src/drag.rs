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
    ($($t:tt),* $(,)?) => {
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
    ($($x:expr => $y:expr),* $(,)?) => {
        use super::*;

        use $crate::{
            consts::FRAC_PI_8,
            error::Result,
            physics::DragFunction,
            simulation::Simulation,
            units::{ReciprocalLength, Velocity},
        };

        use std::sync::Mutex;

        pub const SIZE: usize = count!($($x),*);
        pub static TABLE: Mutex<Option<Table<{ SIZE }>>> = Mutex::new(None);

        #[derive(Debug)]
        pub struct Drag {
            table: Table<{ SIZE }>,
        }

        impl DragFunction for Drag {
            fn new(simulation: &Simulation<Self>) -> Self {
                Self {
                    table: Table {
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
                    }
                }
            }
            fn cd(&self, velocity: Velocity) -> Result<ReciprocalLength> {
                self.table.lerp(velocity)
            }
        }
    };
}
use table;

#[derive(Debug)]
pub struct Table<const N: usize> {
    x: [Velocity; N],
    y: [ReciprocalLength; N],
}

impl<const N: usize> Table<{ N }> {
    pub fn lerp(&self, x: Velocity) -> Result<ReciprocalLength> {
        // Find values in table to interpolate
        let i = self.binary_search(x);
        let j = i + 1;
        if j == N {
            return Err(Error::Velocity(x));
        };
        let (x0, y0) = (self.x[i], self.y[i]);
        let (x1, y1) = (self.x[j], self.y[j]);

        // Linear interpolation
        let y = y0 + (x - x0) * ((y1 - y0) / (x1 - x0));
        // let y = (y0 * (x1 - x0)) / (x1 - x0) + (y1 * (x - x0) - y0 * (x - x0)) / (x1 - x0);
        // let y = (y1 * x - y1 * x0 - y0 * x + y0 * x0 + y0 * x1 - y0 * x0) / (x1 - x0);
        // let y = (y0 * (x1 - x) + y1 * (x - x0)) / (x1 - x0);
        Ok(y)
    }

    pub fn linear_search(&self, x: Velocity) -> usize {
        let mut index = 0;
        while index < N {
            if self.x[index] >= x {
                break;
            }
            index += 1;
        }
        index - 1
    }

    pub fn binary_search(&self, x: Velocity) -> usize {
        let mut low = 0;
        let mut high = N;
        while low < high {
            let mid = low + ((high - low) >> 1);
            if self.x[mid] < x {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        high - 1
    }

    pub fn experimental_search(&self, x: Velocity) -> usize {
        let mut index = 0;
        let mut len = N;
        while len > 1 {
            let half = len >> 1;
            let mid = index + half;
            if self.x[mid] < x {
                index = mid
            };
            len -= half;
        }
        index
    }
}
