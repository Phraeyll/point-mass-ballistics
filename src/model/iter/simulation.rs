use nalgebra::Vector3;

use super::{packet::*, physics::*};
use crate::{
    model::core::{Projectile, Scope, Shooter, Simulation},
    util::*,
};

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
impl Simulation {
    pub fn iter(&self) -> IterSimulation<'_> {
        IterSimulation {
            simulation: self,
            position: self.absolute_projectile_position(),
            velocity: self.absolute_projectile_velocity(),
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
impl TimeStep for IterSimulation<'_> {
    fn delta_time(&self) -> Numeric {
        self.simulation.time_step
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
    type Item = Packet<'s, Simulation>;
    fn next(&mut self) -> Option<Self::Item> {
        // Previous values captured to be returned, so that time 0 can be accounted for
        let &mut Self {
            time,
            position,
            velocity,
            ..
        } = self;

        self.time += self.delta_time();
        self.position += self.delta_position();
        self.velocity += self.delta_velocity();

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

impl ParameterHandles for Simulation {
    fn projectile(&self) -> &Projectile {
        &self.projectile
    }
    fn scope(&self) -> &Scope {
        &self.scope
    }
    fn shooter(&self) -> &Shooter {
        &self.shooter
    }
}
impl GetMeasurement for IterSimulation<'_> {
    fn get_velocity(&self) -> Vector3<Numeric> {
        self.velocity
    }
    fn get_position(&self) -> Vector3<Numeric> {
        self.position
    }
    fn get_time(&self) -> Numeric {
        self.time
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
        self.simulation.projectile.bc().table()
    }
    fn wind_velocity(&self) -> Vector3<Numeric> {
        // Velocity vector of wind, only horizontal at the moment
        // Does not adjust according to line of sight, since most would measure wind
        // along relative bearing - I don't think many would factor in a 'downhill' wind for example
        // This would be interresting to think of, however.
        self.simulation
            .wind
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
impl Coriolis for IterSimulation<'_> {
    fn coriolis_flag(&self) -> bool {
        self.simulation.flags.coriolis()
    }
    fn omega(&self) -> Vector3<Numeric> {
        self.simulation.shooter.omega()
    }
}
