pub use crate::{
    physics::{DragFunction, DragInit},
    units::{ReciprocalLength, Velocity},
};

use std::ops::{Add, Deref, Div, Mul, Sub};

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
        };

        const SIZE: usize = count!($($x),*);

        type Tabl = Table<{ SIZE }, Velocity, ReciprocalLength>;

        #[derive(Debug)]
        pub struct Drag(Tabl);

        impl DragInit for Drag {
            fn new(simulation: &Simulation<Self>) -> Self {
                Self(Table {
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
                })
            }
        }

        impl Deref for Drag {
            type Target = Tabl;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
use table;

#[derive(Debug)]
pub struct Table<const N: usize, X, Y> {
    x: [X; N],
    y: [Y; N],
}

impl<const N: usize, T> DragFunction for T
where
    T: Deref<Target = Table<N, Velocity, ReciprocalLength>>,
{
    fn cd(&self, velocity: Velocity) -> ReciprocalLength {
        self.lerp(velocity)
    }
}

impl<const N: usize, X, Y> Table<N, X, Y> {
    pub fn lerp(&self, x: X) -> Y
    where
        X: Copy + PartialOrd + Sub,
        Y:
            Copy
                + Sub
                + Add<
                    <<X as Sub>::Output as Mul<
                        <<Y as Sub>::Output as Div<<X as Sub>::Output>>::Output,
                    >>::Output,
                    Output = Y,
                >,
        <Y as Sub>::Output: Div<<X as Sub>::Output>,
        <X as Sub>::Output: Mul<<<Y as Sub>::Output as Div<<X as Sub>::Output>>::Output>,
    {
        // Find values in table to interpolate
        let j = search(&self.x, x);

        // Bound to lowest index
        if j == 0 {
            return self.y[j];
        }

        let i = j - 1;

        // Bound to highest index
        if j == N {
            return self.y[i];
        };

        let (x0, y0) = (self.x[i], self.y[i]);
        let (x1, y1) = (self.x[j], self.y[j]);

        // Linear interpolation
        let y = y0 + (x - x0) * ((y1 - y0) / (x1 - x0));
        // let y = (y0 * (x1 - x0)) / (x1 - x0) + (y1 * (x - x0) - y0 * (x - x0)) / (x1 - x0);
        // let y = (y1 * x - y1 * x0 - y0 * x + y0 * x0 + y0 * x1 - y0 * x0) / (x1 - x0);
        // let y = (y0 * (x1 - x) + y1 * (x - x0)) / (x1 - x0);
        y
    }
}

pub fn search<T>(slice: &[T], x: T) -> usize
where
    T: PartialOrd,
{
    let mut left = 0;
    let mut right = slice.len();
    while left < right {
        let size = right - left; // right == left + size
        let half = size / 2; // half < size
        let mid = left + half; // mid >= left && mid < right
        // SAFETY: mid is guaranteed to be in bounds by following:
        // 1.) invariants listed above in comments (mid >= left && mid < right)
        // 2.) initial assignments above loop (mid >= 0, mid < len)
        // 3.) invariants listed below in variable assignments (left can only grow, right can only shrink)
        // 4.) loop condition maintains invariants; not entered when right <= left
        if *unsafe { slice.get_unchecked(mid) } < x {
            left = mid + 1; // left >= previous
        } else {
            right = mid; // right < previous
        }
    }
    right
}
