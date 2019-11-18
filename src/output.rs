use crate::{
    util::{
        foot_per_second, foot_pound, inch, joule, kilogram, meter, meter_per_second, moa,
        nalgebra_helpers::*, radian, yard, Angle, Energy, Length, Numeric, Velocity,
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
    fn time(&self) -> Numeric {
        self.time
    }
    fn velocity(&self) -> Numeric {
        Velocity::new::<meter_per_second>(self.velocity.norm()).get::<foot_per_second>()
    }
    fn energy(&self) -> Numeric {
        Energy::new::<joule>(
            self.simulation.projectile.mass().get::<kilogram>() * self.velocity.norm().powf(2.0)
                / 2.0,
        )
        .get::<foot_pound>()
    }
    // Positions relative to line of sight (shooter_pitch)
    fn distance(&self) -> Numeric {
        Length::new::<meter>(self.relative_position().x).get::<yard>()
    }
    fn elevation(&self) -> Numeric {
        Length::new::<meter>(self.relative_position().y).get::<inch>()
    }
    fn windage(&self) -> Numeric {
        Length::new::<meter>(self.relative_position().z).get::<inch>()
    }
    fn moa(&self) -> Numeric {
        Angle::new::<radian>(self.relative_position().angle(&Vector3::x_axis())).get::<moa>()
    }
    fn vertical_moa(&self, tolerance: Numeric) -> Numeric {
        self.offset_vertical_moa(0.0, Length::new::<inch>(tolerance).get::<meter>())
            .get::<moa>()
    }
    fn horizontal_moa(&self, tolerance: Numeric) -> Numeric {
        self.offset_horizontal_moa(0.0, Length::new::<inch>(tolerance).get::<meter>())
            .get::<moa>()
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

        Angle::new::<radian>(sign * position.angle(&desired))
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

        Angle::new::<radian>(sign * position.angle(&desired))
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
