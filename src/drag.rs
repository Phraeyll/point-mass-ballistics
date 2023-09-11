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

pub type Entry = (Velocity, ReciprocalLength);

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

        #[derive(Debug)]
        pub struct Drag {
            table: [Entry; { count!($($x),*) }],
        }

        impl DragFunction for Drag {
            fn new(simulation: &Simulation<Self>) -> Self {
                Self {
                    table: [
                        $((
                            $x * simulation.atmosphere.sound_velocity(),
                            -($y * FRAC_PI_8) * simulation.atmosphere.rho() / simulation.projectile.bc()
                        )),*
                    ]
                }
            }
            fn cd(&self, velocity: Velocity) -> Result<ReciprocalLength> {
                lerp(&self.table, velocity)
            }
        }
    };
}
use table;

pub fn lerp(slice: &[Entry], x: Velocity) -> Result<ReciprocalLength> {
    // Find values in table to interpolate
    let i = binary_search(slice, x);
    let j = i + 1;
    if j == slice.len() {
        return Err(Error::Velocity(x));
    };
    let (x0, y0) = (slice[i].0, slice[i].1);
    let (x1, y1) = (slice[j].0, slice[j].1);

    // Linear interpolation
    let y = y0 + (x - x0) * ((y1 - y0) / (x1 - x0));
    // let y = (y0 * (x1 - x0)) / (x1 - x0) + (y1 * (x - x0) - y0 * (x - x0)) / (x1 - x0);
    // let y = (y1 * x - y1 * x0 - y0 * x + y0 * x0 + y0 * x1 - y0 * x0) / (x1 - x0);
    // let y = (y0 * (x1 - x) + y1 * (x - x0)) / (x1 - x0);
    Ok(y)
}

pub fn linear_search(slice: &[Entry], x: Velocity) -> usize {
    let mut index = 0;
    while index < slice.len() {
        if slice[index].0 >= x {
            break;
        }
        index += 1;
    }
    index - 1
}

pub fn binary_search(slice: &[Entry], x: Velocity) -> usize {
    let mut low = 0;
    let mut high = slice.len();
    while low < high {
        let mid = low + ((high - low) >> 1);
        if slice[mid].0 < x {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    high - 1
}

pub fn experimental_search(slice: &[Entry], x: Velocity) -> usize {
    let mut index = 0;
    let mut len = slice.len();
    while len > 1 {
        let half = len >> 1;
        let mid = index + half;
        if slice[mid].0 < x {
            index = mid
        };
        len -= half;
    }
    index
}
