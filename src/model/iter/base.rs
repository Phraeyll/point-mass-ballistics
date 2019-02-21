use nalgebra::Vector3;

use crate::model::core::{Atmosphere, Flags, Projectile, Scope, Shooter, Wind};
use crate::util::*;

pub trait GetMeasurement {
    fn s_velocity(&self) -> Vector3<Numeric>;
    fn s_position(&self) -> Vector3<Numeric>;
    fn s_time(&self) -> Numeric;
}
pub trait SimulationHandle {
    type Simulation: ParameterHandles;
    fn simulation(&self) -> &Self::Simulation;
}
pub trait ParameterHandles {
    fn flags(&self) -> &Flags;
    fn projectile(&self) -> &Projectile;
    fn scope(&self) -> &Scope;
    fn shooter(&self) -> &Shooter;
    fn atmosphere(&self) -> &Atmosphere;
    fn wind(&self) -> &Wind;
    fn time_step(&self) -> Numeric;
}
pub trait InitIterator<'s>
where
    Self: ParameterHandles,
{
    type Iter;
    fn iter(&'s self) -> Self::Iter;
    // Rotated velocity vector, accounts for muzzle/shooter pitch, and yaw (bearing)
    // Start with velocity value along X unit vector
    fn absolute_projectile_velocity(&self) -> Vector3<Numeric> {
        self.projectile()
            .velocity(&self.scope())
            .pivot_x(self.shooter().roll())
            .pivot_z(self.shooter().pitch())
            .pivot_y(self.shooter().yaw())
    }
    // Projectiles position relative to scope
    fn absolute_projectile_position(&self) -> Vector3<Numeric> {
        -self
            .scope()
            .position()
            .pivot_x(-self.scope().roll())
            .pivot_x(self.shooter().roll())
            .pivot_z(self.shooter().pitch())
            .pivot_y(self.shooter().yaw())
    }
}
