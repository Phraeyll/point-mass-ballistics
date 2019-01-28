use nalgebra::Vector3;

use super::packet::*;
use crate::model::core::Simulation;
use crate::util::*;

// Iterator over PointMassModel, steps through time and adjust position and velocity vectors
// Has reference to current simulation model for calculations
// Item lifetime also timed to this lifetime
#[derive(Debug)]
pub struct IterSimulation<'s> {
    simulation: &'s Simulation, // Reference to model used for calculations
    position: Vector3<Numeric>, // Position (m)
    velocity: Vector3<Numeric>, // Velocity (m/s)
    time: Numeric,              // Position in time (s)
}
// Ref iter
impl Simulation {
    pub fn iter(&self) -> IterSimulation {
        IterSimulation {
            simulation: self,
            position: self.absolute_projectile_position(),
            velocity: self.absolute_projectile_velocity(),
            time: 0.0,
        }
    }
    // Rotated velocity vector, accounts for muzzle/shooter pitch, and yaw (bearing)
    // Start with velocity value along X unit vector
    pub(crate) fn absolute_projectile_velocity(&self) -> Vector3<Numeric> {
        self.projectile
            .velocity(&self.scope)
            .pivot_x(self.shooter.roll())
            .pivot_z(self.shooter.pitch())
            .pivot_y(self.shooter.yaw())
    }
    // Projectiles position relative to scope
    pub(crate) fn absolute_projectile_position(&self) -> Vector3<Numeric> {
        -self
            .scope
            .position()
            .pivot_x(self.shooter.roll())
            .pivot_x(-self.scope.roll())
            .pivot_z(self.shooter.pitch())
            .pivot_y(self.shooter.yaw())
    }
}
// Create an new iterator over Simulation
impl<'s> IntoIterator for &'s Simulation {
    type Item = <IterSimulation<'s> as Iterator>::Item;
    type IntoIter = IterSimulation<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Produce new 'packet', based on drag, coriolis acceleration, and gravity
// Contains time, position, and velocity of projectile, and reference to simulation used
impl<'s> Iterator for IterSimulation<'s> {
    type Item = Packet<'s>;
    fn next(&mut self) -> Option<Self::Item> {
        // Previous values captured to be returned, so that time 0 can be accounted for
        let time = Newtonian::time(self);
        let position = Newtonian::position(self);
        let velocity = Newtonian::velocity(self);

        self.increment_time();
        self.increment_position();
        self.increment_velocity();

        // Only continue iteration for changing 'forward' positions
        // Old check for norm may show up in false positives - norm could be same for 'valid' velocities
        // that are changing direction, but could still be traversion forward - norm loses information
        // It is only a magnitude.  It could be that the norm is the same for two different velocities
        // that are still moving forward, just at different angles
        //
        // This position check is still bad, however, as position may take a few ticks to change.
        // For practical purposes, this still may suffice.  I want to take this check out eventually, and
        // somehow allow caller to decide when to halt, ie, through filtering adaptors, although am not sure
        // how to check previous iteration values in standard iterator adaptors.
        if Newtonian::position(self).x != position.x {
            Some(Self::Item {
                simulation: &self.simulation,
                time,
                position,
                velocity,
            })
        } else {
            None
        }
    }
}

trait Newtonian {
    fn acceleration(&self) -> Vector3<Numeric>;

    fn increment_time(&mut self);
    fn time(&self) -> Numeric;
    fn delta_time(&self) -> Numeric;

    // 'Second Equation of Motion'
    fn increment_position(&mut self);
    fn position(&self) -> Vector3<Numeric>;
    fn delta_position(&self) -> Vector3<Numeric> {
        self.velocity() * self.delta_time()
            + 0.5 * (self.acceleration() * self.delta_time().powf(2.0))
    }

    // 'First Equation of Motion'
    fn increment_velocity(&mut self);
    fn velocity(&self) -> Vector3<Numeric>;
    fn delta_velocity(&self) -> Vector3<Numeric> {
        self.acceleration() * self.delta_time()
    }
}

trait Drag
where
    Self: Newtonian,
{
    fn drag_flag(&self) -> bool;
    fn projectile_mass(&self) -> Numeric;
    fn projectile_area(&self) -> Numeric;
    fn i(&self) -> Numeric;
    fn cd_table(&self) -> &FloatMap<Numeric>;
    fn wind_velocity(&self) -> Vector3<Numeric>;
    fn speed_of_sound(&self) -> Numeric;
    fn rho(&self) -> Numeric;

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

trait Coriolis
where
    Self: Newtonian,
{
    fn coriolis_flag(&self) -> bool;
    fn omega(&self) -> Vector3<Numeric>;
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

pub trait Gravity {
    fn gravity_flag(&self) -> bool;
    fn gravity(&self) -> Vector3<Numeric>;
    fn gravity_acceleration(&self) -> Vector3<Numeric> {
        if self.gravity_flag() {
            self.gravity()
        } else {
            Vector3::zeros()
        }
    }
}

impl Newtonian for IterSimulation<'_> {
    fn acceleration(&self) -> Vector3<Numeric> {
        self.coriolis_acceleration() + self.drag_acceleration() + self.gravity_acceleration()
    }
    fn delta_time(&self) -> Numeric {
        self.simulation.time_step
    }
    fn increment_time(&mut self) {
        self.time += self.delta_time();
    }
    fn increment_position(&mut self) {
        self.position += self.delta_position();
    }
    fn increment_velocity(&mut self) {
        self.velocity += self.delta_velocity();
    }
    fn time(&self) -> Numeric {
        self.time
    }
    fn position(&self) -> Vector3<Numeric> {
        self.position
    }
    fn velocity(&self) -> Vector3<Numeric> {
        self.velocity
    }
}
impl Coriolis for IterSimulation<'_> {
    fn coriolis_flag(&self) -> bool {
        self.simulation.flags.coriolis()
    }
    fn omega(&self) -> Vector3<Numeric> {
        self.simulation.shooter.omega()
    }
}
impl Drag for IterSimulation<'_> {
    fn drag_flag(&self) -> bool {
        self.simulation.flags.drag()
    }
    fn projectile_mass(&self) -> Numeric {
        self.simulation.projectile.mass()
    }
    fn projectile_area(&self) -> Numeric {
        self.simulation.projectile.area()
    }
    fn i(&self) -> Numeric {
        self.simulation.projectile.i()
    }
    fn cd_table(&self) -> &FloatMap<Numeric> {
        self.simulation.projectile.bc.table()
    }
    fn wind_velocity(&self) -> Vector3<Numeric> {
    // Velocity vector of wind, only horizontal at the moment
    // Does not adjust according to line of sight, since most would measure wind
    // along relative bearing - I don't think many would factor in a 'downhill' wind for example
    // This would be interresting to think of, however.
        self.simulation.wind
            .velocity()
            .pivot_x(self.simulation.shooter.roll())
            .pivot_z(self.simulation.shooter.pitch())
            .pivot_y(self.simulation.shooter.yaw())
    }
    fn speed_of_sound(&self) -> Numeric {
        self.simulation.atmosphere.speed_of_sound()
    }
    fn rho(&self) -> Numeric {
        self.simulation.atmosphere.rho()
    }
}
impl Gravity for IterSimulation<'_> {
    fn gravity_flag(&self) -> bool {
        self.simulation.flags.gravity()
    }
    fn gravity(&self) -> Vector3<Numeric> {
        self.simulation.shooter.gravity()
    }
}