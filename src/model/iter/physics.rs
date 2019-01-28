use nalgebra::Vector3;

use crate::model::core::{Atmosphere, Flags, Projectile, Scope, Shooter, Wind};
use crate::util::*;

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
pub trait InitIterator
where
    Self: ParameterHandles,
{
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
            .pivot_x(self.shooter().roll())
            .pivot_x(-self.scope().roll())
            .pivot_z(self.shooter().pitch())
            .pivot_y(self.shooter().yaw())
    }
}
pub trait GetVelocity {
    fn velocity(&self) -> Vector3<Numeric>;
}
impl<I> Newtonian for I
where
    I: Coriolis + Drag + Gravity,
{
    fn acceleration(&self) -> Vector3<Numeric> {
        self.coriolis_acceleration() + self.drag_acceleration() + self.gravity_acceleration()
    }
}
pub trait Newtonian
where
    Self: SimulationHandle,
    Self: GetVelocity,
{
    fn acceleration(&self) -> Vector3<Numeric>;
    fn delta_time(&self) -> Numeric {
        self.simulation().time_step()
    }
    // 'Second Equation of Motion'
    fn delta_position(&self) -> Vector3<Numeric> {
        self.velocity() * self.delta_time()
            + 0.5 * (self.acceleration() * self.delta_time().powf(2.0))
    }
    // 'First Equation of Motion'
    fn delta_velocity(&self) -> Vector3<Numeric> {
        self.acceleration() * self.delta_time()
    }
}
pub trait Drag
where
    Self: GetVelocity,
    Self: SimulationHandle,
{
    fn drag_flag(&self) -> bool {
        self.simulation().flags().drag()
    }
    fn projectile_mass(&self) -> Numeric {
        self.simulation().projectile().mass()
    }
    fn projectile_area(&self) -> Numeric {
        self.simulation().projectile().area()
    }
    fn i(&self) -> Numeric {
        self.simulation().projectile().i()
    }
    fn cd_table(&self) -> &FloatMap<Numeric> {
        self.simulation().projectile().bc.table()
    }
    fn wind_velocity(&self) -> Vector3<Numeric> {
        // Velocity vector of wind, only horizontal at the moment
        // Does not adjust according to line of sight, since most would measure wind
        // along relative bearing - I don't think many would factor in a 'downhill' wind for example
        // This would be interresting to think of, however.
        self.simulation()
            .wind()
            .velocity()
            .pivot_x(self.simulation().shooter().roll())
            .pivot_z(self.simulation().shooter().pitch())
            .pivot_y(self.simulation().shooter().yaw())
    }
    fn speed_of_sound(&self) -> Numeric {
        self.simulation().atmosphere().speed_of_sound()
    }
    fn rho(&self) -> Numeric {
        self.simulation().atmosphere().rho()
    }
    fn drag_acceleration(&self) -> Vector3<Numeric> {
        if self.drag_flag() {
            // Acceleration from drag force and gravity (F = ma)
            self.drag_force() / self.projectile_mass()
        } else {
            Vector3::zeros()
        }
    }
    // Velocity vector, after impact from wind (actually from drag, not "being blown")
    // This is why the velocity from wind is subtracted, and vv is not used to find next velocity
    fn vv(&self) -> Vector3<Numeric> {
        self.velocity() - self.wind_velocity()
    }
    // Velocity relative to speed of sound (c), with given atmospheric conditions
    fn mach(&self) -> Numeric {
        self.velocity().norm() / self.speed_of_sound()
    }
    // Coefficient of drag, as defined by a standard projectile depending on drag table used
    fn cd(&self) -> Numeric {
        self.i() * self.cd_table().lerp(self.mach()).expect("cd")
    }
    // Force of drag for given projectile, at given mach speed, with given conditions
    // Drag force is proportional to square of velocity and area of projectile, scaled
    // by a coefficient at mach speeds (approximately)
    fn drag_force(&self) -> Vector3<Numeric> {
        -0.5 * self.rho() * self.vv() * self.vv().norm() * self.cd() * self.projectile_area()
    }
}
pub trait Coriolis
where
    Self: GetVelocity,
    Self: SimulationHandle,
{
    fn coriolis_flag(&self) -> bool {
        self.simulation().flags().coriolis()
    }
    fn omega(&self) -> Vector3<Numeric> {
        self.simulation().shooter().omega()
    }
    // Coriolis/Eotovos acceleration vector.  Accounts for Left/Right drift due to Earth's spin
    // This drift is always right (+z relative) in the northern hemisphere, regardless of initial bearing
    // This drive is always left (-z relative) in the southern hemisphere, regardless of initial bearing
    // Also accounts for elevation changes when launching projectils East/West, regardless of hemisphere
    // Bearing East results in higher elevation (+y absolute/relative)
    // Bearing West results in lower elevation (-y relative/absolute)
    fn coriolis_acceleration(&self) -> Vector3<Numeric> {
        if self.coriolis_flag() {
            -2.0 * self.omega().cross(&self.velocity())
        } else {
            Vector3::zeros()
        }
    }
}
pub trait Gravity
where
    Self: SimulationHandle,
{
    fn gravity_flag(&self) -> bool {
        self.simulation().flags().gravity()
    }
    fn gravity(&self) -> Vector3<Numeric> {
        self.simulation().shooter().gravity()
    }
    fn gravity_acceleration(&self) -> Vector3<Numeric> {
        if self.gravity_flag() {
            self.gravity()
        } else {
            Vector3::zeros()
        }
    }
}
