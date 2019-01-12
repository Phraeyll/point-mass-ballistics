use nalgebra::{Rotation3, Vector3};

use super::*;

pub trait PitchYawRoll {
    fn pivot_z(&self, angle: Numeric) -> Self;
    fn pivot_y(&self, angle: Numeric) -> Self;
    fn pivot_x(&self, angle: Numeric) -> Self;
}
impl PitchYawRoll for Vector3<Numeric> {
    fn pivot_z(&self, angle: Numeric) -> Self {
        Rotation3::from_axis_angle(&Vector3::z_axis(), angle) * self
    }
    fn pivot_y(&self, angle: Numeric) -> Self {
        Rotation3::from_axis_angle(&Vector3::y_axis(), angle) * self
    }
    fn pivot_x(&self, angle: Numeric) -> Self {
        Rotation3::from_axis_angle(&Vector3::x_axis(), angle) * self
    }
}
