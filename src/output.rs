use crate::{util::*, Simulation};

use nalgebra::Vector3;

// Output of iteration, need a better name to encapsulate a moving projectile
#[derive(Debug)]
pub struct Packet<'t> {
    pub(crate) simulation: &'t Simulation<'t>, //Simulation this came from, used for various calculations
    pub(crate) time: Numeric,                  // Position in time (s)
    pub(crate) position: Vector3<Numeric>,     // Position (m)
    pub(crate) velocity: Vector3<Numeric>,     // Velocity (m/s)
}

impl Measurements for Packet<'_> {
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
        self.offset_vertical_moa(0.0, Length::Inches(tolerance).to_meters().to_num())
            .to_minutes()
            .to_num()
    }
    fn horizontal_moa(&self, tolerance: Numeric) -> Numeric {
        self.offset_horizontal_moa(0.0, Length::Inches(tolerance).to_meters().to_num())
            .to_minutes()
            .to_num()
    }
    // During the simulation, the velocity of the projectile is rotated to allign with
    // the shooter's bearing (azimuth and line of sight)
    // This function returns the position rotated back to the initial frame of reference
    // This is used during zero'ing and is output in the drop table
    fn relative_position(&self) -> Vector3<Numeric> {
        self.position
            .pivot_y(-self.simulation.shooter.yaw())
            .pivot_z(-self.simulation.shooter.pitch())
            .pivot_x(-self.simulation.shooter.roll())
    }
    // This gives adjustment - opposite sign relative to desired offset
    // Always done in meters for now, due to relative_position()
    fn offset_vertical_moa(&self, offset: Numeric, tolerance: Numeric) -> Angle {
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
    // Always done in meters for now, due to relative_position()
    fn offset_horizontal_moa(&self, offset: Numeric, tolerance: Numeric) -> Angle {
        let sign = if self.relative_position().z >= (offset - tolerance) {
            -1.0
        } else {
            1.0
        };

        let position = Vector3::new(self.relative_position().x, 0.0, self.relative_position().z);
        let desired = Vector3::new(self.relative_position().x, 0.0, offset);

        Angle::Radians(sign * position.angle(&desired))
    }
}

pub trait Measurements {
    fn time(&self) -> Numeric;
    fn velocity(&self) -> Numeric;
    fn energy(&self) -> Numeric;
    fn distance(&self) -> Numeric;
    fn elevation(&self) -> Numeric;
    fn windage(&self) -> Numeric;
    fn moa(&self) -> Numeric;
    fn vertical_moa(&self, tolerance: Numeric) -> Numeric;
    fn horizontal_moa(&self, tolerance: Numeric) -> Numeric;
    fn relative_position(&self) -> Vector3<Numeric>;
    fn offset_vertical_moa(&self, offset: Numeric, tolerance: Numeric) -> Angle;
    fn offset_horizontal_moa(&self, offset: Numeric, tolerance: Numeric) -> Angle;
}