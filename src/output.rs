use crate::{
    util::{
        foot_per_second, foot_pound, inch, joule, kilogram, length, meter, meter_per_second, moa,
        nalgebra_helpers::*, radian, second, typenum::P2, Angle, Conversion, Energy, Length,
        Numeric, Time, Velocity,
    },
    Simulation,
};

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
    fn time(&self) -> Time {
        Time::new::<second>(self.time)
    }
    fn velocity(&self) -> Velocity {
        Velocity::new::<meter_per_second>(self.velocity.norm())
    }
    fn energy(&self) -> Energy {
        0.5 * self.simulation.projectile.mass()
            * Velocity::new::<meter_per_second>(self.velocity.norm()).powi(P2::new())
    }
    // Positions relative to line of sight (shooter_pitch)
    fn distance(&self) -> Length {
        Length::new::<meter>(self.relative_position().x)
    }
    fn elevation(&self) -> Length {
        Length::new::<meter>(self.relative_position().y)
    }
    fn windage(&self) -> Length {
        Length::new::<meter>(self.relative_position().z)
    }
    fn moa(&self) -> Angle {
        Angle::new::<radian>(self.relative_position().angle(&Vector3::x_axis()))
    }
    fn vertical_moa(&self, tolerance: Length) -> Angle {
        self.offset_vertical_moa(Length::new::<meter>(0.0), tolerance)
    }
    fn horizontal_moa(&self, tolerance: Length) -> Angle {
        self.offset_horizontal_moa(Length::new::<meter>(0.0), tolerance)
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
    fn offset_vertical_moa(&self, offset: Length, tolerance: Length) -> Angle {
        let sign = if self.elevation() >= (offset - tolerance) {
            -1.0
        } else {
            1.0
        };

        let position = Vector3::new(
            self.distance().get::<meter>(),
            self.elevation().get::<meter>(),
            0.0,
        );
        let desired = Vector3::new(self.distance().get::<meter>(), offset.get::<meter>(), 0.0);

        Angle::new::<radian>(sign * position.angle(&desired))
    }
    // This gives adjustment - opposite sign relative to desired offset
    // Always done in meters for now, due to relative_position()
    fn offset_horizontal_moa(&self, offset: Length, tolerance: Length) -> Angle {
        let sign = if self.distance() >= (offset - tolerance) {
            -1.0
        } else {
            1.0
        };

        let position = Vector3::new(
            self.distance().get::<meter>(),
            0.0,
            self.windage().get::<meter>(),
        );
        let desired = Vector3::new(self.distance().get::<meter>(), 0.0, offset.get::<meter>());

        Angle::new::<radian>(sign * position.angle(&desired))
    }
}

pub trait Measurements {
    fn time(&self) -> Time;
    fn velocity(&self) -> Velocity;
    fn energy(&self) -> Energy;
    fn distance(&self) -> Length;
    fn elevation(&self) -> Length;
    fn windage(&self) -> Length;
    fn moa(&self) -> Angle;
    fn vertical_moa(&self, tolerance: Length) -> Angle;
    fn horizontal_moa(&self, tolerance: Length) -> Angle;
    fn relative_position(&self) -> Vector3<Numeric>;
    fn offset_vertical_moa(&self, offset: Length, tolerance: Length) -> Angle;
    fn offset_horizontal_moa(&self, offset: Length, tolerance: Length) -> Angle;
}
