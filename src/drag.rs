pub use crate::physics::DragFunction;

use std::ops::{Add, Div, Mul, Sub};

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
            simulation::Simulation,
            units::{ReciprocalLength, Velocity},
        };

        #[derive(Debug)]
        pub struct Drag {
            x: [Velocity; { count!($($x),*) }],
            y: [ReciprocalLength; { count!($($y),*) }],
        }

        impl DragFunction for Drag {
            fn new(simulation: &Simulation<Self>) -> Self {
                Self {
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
            fn cd(&self, velocity: Velocity) -> ReciprocalLength {
                lerp(&self.x, &self.y, velocity)
            }
        }
    };
}
use table;

pub fn lerp<T, U>(xs: &[T], ys: &[U], x: T) -> U
where
    T: Copy + PartialOrd + Sub,
    U: Copy + Sub + Add<<<T as Sub>::Output as Mul<<<U as Sub>::Output as Div<<T as Sub>::Output>>::Output>>::Output, Output = U>,
    <U as Sub>::Output: Div<<T as Sub>::Output>,
    <T as Sub>::Output: Mul<<<U as Sub>::Output as Div<<T as Sub>::Output>>::Output>,
{
    // Find values in table to interpolate
    let j = search(xs, x);

    // Bound to lowest index
    if j == 0 {
        return ys[j];
    }

    let i = j - 1;

    // Bound to highest index
    if j == ys.len() {
        return ys[i];
    };

    let (x0, y0) = (xs[i], ys[i]);
    let (x1, y1) = (xs[j], ys[j]);

    // Linear interpolation
    let y = y0 + (x - x0) * ((y1 - y0) / (x1 - x0));
    // let y = (y0 * (x1 - x0)) / (x1 - x0) + (y1 * (x - x0) - y0 * (x - x0)) / (x1 - x0);
    // let y = (y1 * x - y1 * x0 - y0 * x + y0 * x0 + y0 * x1 - y0 * x0) / (x1 - x0);
    // let y = (y0 * (x1 - x) + y1 * (x - x0)) / (x1 - x0);
    y
}

pub fn search<T>(slice: &[T], x: T) -> usize
where
    T: PartialOrd,
{
    let mut low = 0;
    let mut high = slice.len();
    while low < high {
        let mid = low + ((high - low) >> 1);
        if slice[mid] < x {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    high
}
