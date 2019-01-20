use nalgebra::{Rotation3, Vector3};

use crate::util::*;

pub trait PitchYawRoll {
    fn pivot_z(&self, angle: Angle) -> Self;
    fn pivot_y(&self, angle: Angle) -> Self;
    fn pivot_x(&self, angle: Angle) -> Self;
    fn un_pivot_z(&self, angle: Angle) -> Self;
    fn un_pivot_y(&self, angle: Angle) -> Self;
    fn un_pivot_x(&self, angle: Angle) -> Self;
}
impl PitchYawRoll for Vector3<Numeric> {
    fn pivot_z(&self, angle: Angle) -> Self {
        Rotation3::from_axis_angle(&Vector3::z_axis(), angle.to_radians().to_num()) * self
    }
    fn pivot_y(&self, angle: Angle) -> Self {
        Rotation3::from_axis_angle(&Vector3::y_axis(), angle.to_radians().to_num()) * self
    }
    fn pivot_x(&self, angle: Angle) -> Self {
        Rotation3::from_axis_angle(&Vector3::x_axis(), angle.to_radians().to_num()) * self
    }
    fn un_pivot_z(&self, angle: Angle) -> Self {
        Rotation3::from_axis_angle(&Vector3::z_axis(), -angle.to_radians().to_num()) * self
    }
    fn un_pivot_y(&self, angle: Angle) -> Self {
        Rotation3::from_axis_angle(&Vector3::y_axis(), -angle.to_radians().to_num()) * self
    }
    fn un_pivot_x(&self, angle: Angle) -> Self {
        Rotation3::from_axis_angle(&Vector3::x_axis(), -angle.to_radians().to_num()) * self
    }
}
