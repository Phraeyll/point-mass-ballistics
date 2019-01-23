use nalgebra::Vector3;

use crate::util::*;
use crate::model::core::Simulation;

// Iterator over PointMassModel, steps through time and adjust position and velocity vectors
// Has reference to current simulation model for calculations
// Output Item has this same reference
#[derive(Debug)]
pub struct IterSimulation<'s> {
    simulation: &'s Simulation, // Reference to model used for calculations
    position: Vector3<Numeric>,            // Position (m)
    velocity: Vector3<Numeric>,            // Velocity (m/s)
    time: Numeric,                         // Position in time (s)
}
// Output of iteration, need a better name to encapsulate a moving projectile
#[derive(Debug)]
pub struct Packet<'s> {
    pub(crate) simulation: &'s super::Simulation, //Simulation this came from, used for various calculations
    pub(crate) time: Numeric,                         // Position in time (s)
    pub(crate) position: Vector3<Numeric>,            // Position (m)
    pub(crate) velocity: Vector3<Numeric>,            // Velocity (m/s)
}

// Create an new iterator over Simulation
impl<'s> IntoIterator for &'s Simulation {
    type Item = <IterSimulation<'s> as Iterator>::Item;
    type IntoIter = IterSimulation<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Ref iter
impl Simulation {
    pub fn iter(&self) -> IterSimulation {
        IterSimulation {
            simulation: self,
            position: -self.scope.position(),
            velocity: -self.absolute_projectile_velocity().pivot_z(self.scope.roll()),
            time: 0.0,
        }
    }
}
// Produce new 'packet', based on drag, coriolis acceleration, and gravity
// Contains time, position, and velocity of projectile, and reference to simulation used
impl<'s> Iterator for IterSimulation<'s> {
    type Item = Packet<'s>;
    fn next(&mut self) -> Option<Self::Item> {
        // Previous values captured to be returned, so that time 0 can be accounted for
        let &mut Self {
            time,
            position,
            velocity,
            ..
        } = self;

        // Unwrap time
        let time_step = self.simulation.time_step.to_seconds().to_num();
        // Increment position in time
        self.time += time_step;
        // 'Second Equation of Motion'
        self.position +=
            self.velocity * time_step + (self.acceleration() * time_step.powf(2.0)) / 2.0;
        // 'First Equation of Motion'
        self.velocity += self.acceleration() * time_step;

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
// Calculations used during iteration
impl IterSimulation<'_> {
    // Coriolis/Eotovos acceleration vector.  Accounts for Left/Right drift due to Earth's spin
    // This drift is always right (+z relative) in the northern hemisphere, regardless of initial bearing
    // This drive is always left (-z relative) in the southern hemisphere, regardless of initial bearing
    // Also accounts for elevation changes when launching projectils East/West, regardless of hemisphere
    // Bearing East results in higher elevation (+y absolute/relative)
    // Bearing West results in lower elevation (-y relative/absolute)
    fn coriolis_acceleration(&self) -> Vector3<Numeric> {
        if self.simulation.flags.use_coriolis {
            -2.0 * self
                .simulation
                .conditions
                .other
                .omega()
                .cross(&self.velocity)
        } else {
            Vector3::zeros()
        }
    }
    fn gravity_acceleration(&self) -> Vector3<Numeric> {
        if self.simulation.flags.use_gravity {
            self.simulation.conditions.other.gravity()
        } else {
            Vector3::zeros()
        }
    }
    fn drag_acceleration(&self) -> Vector3<Numeric> {
        if self.simulation.flags.use_drag {
            // Acceleration from drag force and gravity (F = ma)
            self.drag_force() / self.simulation.projectile.mass()
        } else {
            Vector3::zeros()
        }
    }
    fn acceleration(&self) -> Vector3<Numeric> {
        self.coriolis_acceleration() + self.drag_acceleration() + self.gravity_acceleration()
    }
    // Force of drag for given projectile, at given mach speed, with given conditions
    // Drag force is proportional to square of velocity and area of projectile, scaled
    // by a coefficient at mach speeds (approximately)
    fn drag_force(&self) -> Vector3<Numeric> {
        -0.5 * self.simulation.conditions.atmosphere.rho()
            * self.simulation.projectile.area()
            * self.cd()
            * self.simulation.projectile.i()
            * self.vv()
            * self.vv().norm()
    }
    // Coefficient of drag, as defined by a standard projectile depending on drag table used
    fn cd(&self) -> Numeric {
        self.simulation
            .projectile
            .bc
            .table()
            .lerp(self.mach())
            .expect("cd")
    }
    // Velocity relative to speed of sound (c), with given atmospheric conditions
    fn mach(&self) -> Numeric {
        self.velocity.norm() / self.simulation.conditions.atmosphere.speed_of_sound()
    }
    // Velocity vector, after impact from wind (actually from drag, not "being blown")
    // This is why the velocity from wind is subtracted, and vv is not used to find next velocity
    fn vv(&self) -> Vector3<Numeric> {
        self.velocity - self.simulation.absolute_wind_velocity()
    }
}
// Output struct which represents projectiles current position, and velocity
// Basically same values used internally during iteration
// Along with a ref to the simulation which was iterated over
impl Packet<'_> {
    // During the simulation, the velocity of the projectile is rotated to allign with
    // the shooter's bearing (azimuth and line of sight)
    // This function returns the position rotated back to the initial frame of reference
    // This is used during zero'ing and is output in the drop table
    pub fn relative_position(&self) -> Vector3<Numeric> {
        self.position
            .un_pivot_z(self.simulation.conditions.other.line_of_sight)
            .un_pivot_y(self.simulation.conditions.other.corrected_azimuth())
    }
    // This gives adjustment - opposite sign relative to desired offset
    pub(crate) fn offset_vertical_moa(&self, offset: Length, tolerance: Length) -> Angle {
        let offset = offset.to_meters().to_num();
        let tolerance = tolerance.to_meters().to_num();

        let sign = if self.relative_position().y >= (offset - tolerance) {
            -1.0
        } else {
            1.0
        };

        let position = Vector3::new(self.relative_position().x, self.relative_position().y, 0.0);
        let desired = Vector3::new(self.relative_position().x, offset, 0.0);

        Angle::Radians(sign * position.angle(&desired))
    }
    // This gives adjustment - opposite sign relative to desired offset
    pub(crate) fn offset_horizontal_moa(&self, offset: Length, tolerance: Length) -> Angle {
        let offset = offset.to_meters().to_num();
        let tolerance = tolerance.to_meters().to_num();

        let sign = if self.relative_position().z >= (offset - tolerance) {
            1.0
        } else {
            -1.0
        };

        let position = Vector3::new(self.relative_position().x, 0.0, self.relative_position().z);
        let desired = Vector3::new(self.relative_position().x, 0.0, offset);

        Angle::Radians(sign * position.angle(&desired))
    }
}

pub trait Output {
    fn time(&self) -> Numeric;
    fn velocity(&self) -> Numeric;
    fn energy(&self) -> Numeric;
    fn distance(&self) -> Numeric;
    fn elevation(&self) -> Numeric;
    fn windage(&self) -> Numeric;
    fn moa(&self) -> Numeric;
    fn vertical_moa(&self, tolerance: Numeric) -> Numeric;
    fn horizontal_moa(&self, tolerance: Numeric) -> Numeric;
}
// Hard coded Imperial units for now - need to use better library for this eventually
impl Output for Packet<'_> {
    fn time(&self) -> Numeric {
        Time::Seconds(self.time).to_seconds().to_num()
    }
    fn velocity(&self) -> Numeric {
        Velocity::Mps(self.velocity.norm()).to_fps().to_num()
    }
    fn energy(&self) -> Numeric {
        Energy::Joules(self.simulation.projectile.mass() * self.velocity.norm().powf(2.0) / 2.0)
            .to_ftlbs()
            .to_num()
    }
    // Positions relative to line of sight (shooter_pitch)
    fn distance(&self) -> Numeric {
        Length::Meters(self.relative_position().x)
            .to_yards()
            .to_num()
    }
    fn elevation(&self) -> Numeric {
        Length::Meters(self.relative_position().y)
            .to_inches()
            .to_num()
    }
    fn windage(&self) -> Numeric {
        Length::Meters(self.relative_position().z)
            .to_inches()
            .to_num()
    }
    fn moa(&self) -> Numeric {
        Angle::Radians(self.relative_position().angle(&Vector3::x_axis()))
            .to_minutes()
            .to_num()
    }
    fn vertical_moa(&self, tolerance: Numeric) -> Numeric {
        self.offset_vertical_moa(Length::Inches(0.0), Length::Inches(tolerance))
            .to_minutes()
            .to_num()
    }
    fn horizontal_moa(&self, tolerance: Numeric) -> Numeric {
        self.offset_horizontal_moa(Length::Inches(0.0), Length::Inches(tolerance))
            .to_minutes()
            .to_num()
    }
}
