#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

extern crate nalgebra as na;
extern crate ordered_float as of;

pub use dragtables::TableKind::*;
pub mod simulation;
pub mod consts;
mod conversions;
mod dragtables;
