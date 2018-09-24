use nalgebra::{Rotation3, Vector3};
use super::*;

pub trait PitchYawRoll {
    fn pitch(&self, angle: Numeric) -> Self;
    fn yaw(&self, angle: Numeric) -> Self;
    fn roll(&self, angle: Numeric) -> Self;
}
impl PitchYawRoll for Vector3<Numeric> {
    fn pitch(&self, angle: Numeric) -> Self {
        Rotation3::from_axis_angle(&Vector3::z_axis(), angle) * self
    }
    fn yaw(&self, angle: Numeric) -> Self {
        Rotation3::from_axis_angle(&Vector3::y_axis(), angle) * self
    }
    fn roll(&self, angle: Numeric) -> Self {
        Rotation3::from_axis_angle(&Vector3::x_axis(), angle) * self
    }
}
