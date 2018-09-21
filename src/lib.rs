#![feature(rust_2018_preview)]
#[macro_use]
extern crate approx;
#[macro_use]
mod macros;

extern crate nalgebra as na;
extern crate ordered_float as of;
// extern crate num;

pub mod simulation;

mod conversions;
mod dragtables;
mod util;

pub use util::Numeric;
