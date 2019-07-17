use crate::{output::Packet, util::*, Simulation};

use nalgebra::Vector3;

// Iterator over PointMassModel, steps through time and adjust position and velocity vectors
// Has reference to current simulation model for calculations
// Item lifetime also timed to this lifetime
#[derive(Debug)]
pub struct Iter<'t> {
    simulation: &'t Simulation<'t>, // Reference to model used for calculations
    position: Vector3<Numeric>,     // Position (m)
    velocity: Vector3<Numeric>,     // Velocity (m/s)
    time: Numeric,                  // Position in time (s)
}
impl<'t> Simulation<'t> {
    pub fn iter(&self) -> Iter<'_> {
        let position = self.absolute_projectile_position();
        let velocity = self.absolute_projectile_velocity();
        Iter {
            simulation: self,
            position,
            velocity,
            time: 0.0,
        }
    }
    // Rotated velocity vector, accounts for muzzle/shooter pitch, and yaw (bearing)
    // Start with velocity value along X unit vector
    fn absolute_projectile_velocity(&self) -> Vector3<Numeric> {
        self.projectile
            .velocity(&self.scope)
            .pivot_x(self.shooter.roll())
            .pivot_z(self.shooter.pitch())
            .pivot_y(self.shooter.yaw())
    }
    // Projectiles position relative to scope
    fn absolute_projectile_position(&self) -> Vector3<Numeric> {
        -self
            .scope
            .position()
            .pivot_x(-self.scope.roll())
            .pivot_x(self.shooter.roll())
            .pivot_z(self.shooter.pitch())
            .pivot_y(self.shooter.yaw())
    }
}
// Create an new iterator over Simulation
impl<'t> IntoIterator for &'t Simulation<'t> {
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = Iter<'t>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
// Produce new 'packet', based on drag, coriolis acceleration, and gravity
// Contains time, position, and velocity of projectile, and reference to simulation used
impl<'t> Iterator for Iter<'t> {
    type Item = Packet<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        // Previous values captured to be returned, so that time 0 can be accounted for
        let &mut Self {
            time,
            position,
            velocity,
            simulation,
        } = self;

        self.time += simulation.delta_time();
        self.position += simulation.delta_position(&velocity);
        self.velocity += simulation.delta_velocity(&velocity);

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
        if self.position.x != position.x {
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

impl Simulation<'_> {
    fn acceleration(&self, velocity: &Vector3<Numeric>) -> Vector3<Numeric> {
        self.coriolis_acceleration(velocity)
            + self.drag_acceleration(velocity)
            + self.gravity_acceleration()
    }
    fn delta_time(&self) -> Numeric {
        self.time_step
    }
    // 'Second Equation of Motion'
    fn delta_position(&self, velocity: &Vector3<Numeric>) -> Vector3<Numeric> {
        velocity * self.delta_time()
            + 0.5 * (self.acceleration(velocity) * self.delta_time().powf(2.0))
    }
    // 'First Equation of Motion'
    fn delta_velocity(&self, velocity: &Vector3<Numeric>) -> Vector3<Numeric> {
        self.acceleration(velocity) * self.delta_time()
    }
}
