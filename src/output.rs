use crate::{
    util::{
        angle, energy, length, meter, time, typenum::P2, velocity, Angle, DisplayStyle, Energy,
        Length, MyQuantityArguments, Time, Velocity,
    },
    vectors::*,
    Simulation,
};

// Output of iteration, need a better name to encapsulate a moving projectile
#[derive(Debug)]
pub struct Packet<'t> {
    pub(crate) simulation: &'t Simulation, //Simulation this came from, used for various calculations
    pub(crate) time: Time,                 // Position in time (s)
    pub(crate) position: MyVector3<length::Dimension>, // Position (m)
    pub(crate) velocity: MyVector3<velocity::Dimension>, // Velocity (m/s)
}

impl FmtMeasurements for Packet<'_> {
    fn fmt_time<N: time::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
    ) -> MyQuantityArguments<time::Dimension, N> {
        self.time().into_format_args(unit, style)
    }
    fn fmt_velocity<N: velocity::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
    ) -> MyQuantityArguments<velocity::Dimension, N> {
        self.velocity().into_format_args(unit, style)
    }
    fn fmt_energy<N: energy::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
    ) -> MyQuantityArguments<energy::Dimension, N> {
        self.energy().into_format_args(unit, style)
    }
    fn fmt_distance<N: length::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
    ) -> MyQuantityArguments<length::Dimension, N> {
        self.distance().into_format_args(unit, style)
    }
    fn fmt_elevation<N: length::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
    ) -> MyQuantityArguments<length::Dimension, N> {
        self.elevation().into_format_args(unit, style)
    }
    fn fmt_windage<N: length::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
    ) -> MyQuantityArguments<length::Dimension, N> {
        self.windage().into_format_args(unit, style)
    }
    fn fmt_angle<N: angle::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
    ) -> MyQuantityArguments<angle::Dimension, N> {
        self.angle().into_format_args(unit, style)
    }
    fn fmt_vertical_angle<N: angle::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
        tolerance: Length,
    ) -> MyQuantityArguments<angle::Dimension, N> {
        self.vertical_angle(tolerance).into_format_args(unit, style)
    }
    fn fmt_horizontal_angle<N: angle::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
        tolerance: Length,
    ) -> MyQuantityArguments<angle::Dimension, N> {
        self.horizontal_angle(tolerance)
            .into_format_args(unit, style)
    }
}

impl Measurements for Packet<'_> {
    fn time(&self) -> Time {
        self.time
    }
    fn velocity(&self) -> Velocity {
        self.velocity.norm()
    }
    fn energy(&self) -> Energy {
        self.velocity.norm().powi(P2::new()) * self.simulation.projectile.mass() * 0.5
    }
    // Positions relative to line of sight (shooter_pitch)
    fn distance(&self) -> Length {
        self.relative_position().get_x()
    }
    fn elevation(&self) -> Length {
        self.relative_position().get_y()
    }
    fn windage(&self) -> Length {
        self.relative_position().get_z()
    }
    fn angle(&self) -> Angle {
        let compare = MyVector3::new(
            Length::new::<meter>(1.0),
            Length::new::<meter>(0.0),
            Length::new::<meter>(0.0),
        );
        self.relative_position().angle(&compare)
    }
    fn vertical_angle(&self, tolerance: Length) -> Angle {
        self.offset_vertical_angle(Length::new::<meter>(0.0), tolerance)
    }
    fn horizontal_angle(&self, tolerance: Length) -> Angle {
        self.offset_horizontal_angle(Length::new::<meter>(0.0), tolerance)
    }
    // During the simulation, the velocity of the projectile is rotated to allign with
    // the shooter's bearing (azimuth and line of sight)
    // This function returns the position rotated back to the initial frame of reference
    // This is used during zero'ing and is output in the drop table
    fn relative_position(&self) -> MyVector3<length::Dimension> {
        self.position
            .pivot_y(-self.simulation.shooter.yaw())
            .pivot_z(-self.simulation.shooter.pitch())
            .pivot_x(-self.simulation.shooter.roll())
    }
    // This gives adjustment - opposite sign relative to desired offset
    // Always done in meters for now, due to relative_position()
    fn offset_vertical_angle(&self, offset: Length, tolerance: Length) -> Angle {
        let sign = if self.elevation() >= (offset - tolerance) {
            -1.0
        } else {
            1.0
        };

        let position = MyVector3::new(self.distance(), self.elevation(), Length::new::<meter>(0.0));
        let desired = MyVector3::new(self.distance(), offset, Length::new::<meter>(0.0));

        position.angle(&desired) * sign
    }
    // This gives adjustment - opposite sign relative to desired offset
    // Always done in meters for now, due to relative_position()
    fn offset_horizontal_angle(&self, offset: Length, tolerance: Length) -> Angle {
        let sign = if self.windage() >= (offset - tolerance) {
            -1.0
        } else {
            1.0
        };

        let position = MyVector3::new(self.distance(), Length::new::<meter>(0.0), self.windage());
        let desired = MyVector3::new(self.distance(), Length::new::<meter>(0.0), offset);

        position.angle(&desired) * sign
    }
}

pub trait Measurements {
    fn time(&self) -> Time;
    fn velocity(&self) -> Velocity;
    fn energy(&self) -> Energy;
    fn distance(&self) -> Length;
    fn elevation(&self) -> Length;
    fn windage(&self) -> Length;
    fn angle(&self) -> Angle;
    fn vertical_angle(&self, tolerance: Length) -> Angle;
    fn horizontal_angle(&self, tolerance: Length) -> Angle;
    fn relative_position(&self) -> MyVector3<length::Dimension>;
    fn offset_vertical_angle(&self, offset: Length, tolerance: Length) -> Angle;
    fn offset_horizontal_angle(&self, offset: Length, tolerance: Length) -> Angle;
}

pub trait FmtMeasurements {
    fn fmt_time<N: time::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
    ) -> MyQuantityArguments<time::Dimension, N>;
    fn fmt_velocity<N: velocity::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
    ) -> MyQuantityArguments<velocity::Dimension, N>;
    fn fmt_energy<N: energy::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
    ) -> MyQuantityArguments<energy::Dimension, N>;
    fn fmt_distance<N: length::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
    ) -> MyQuantityArguments<length::Dimension, N>;
    fn fmt_elevation<N: length::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
    ) -> MyQuantityArguments<length::Dimension, N>;
    fn fmt_windage<N: length::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
    ) -> MyQuantityArguments<length::Dimension, N>;
    fn fmt_angle<N: angle::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
    ) -> MyQuantityArguments<angle::Dimension, N>;
    fn fmt_vertical_angle<N: angle::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
        tolerance: Length,
    ) -> MyQuantityArguments<angle::Dimension, N>;
    fn fmt_horizontal_angle<N: angle::Unit>(
        &self,
        unit: N,
        style: DisplayStyle,
        tolerance: Length,
    ) -> MyQuantityArguments<angle::Dimension, N>;
}
