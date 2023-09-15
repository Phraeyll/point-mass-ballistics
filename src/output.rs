use crate::{
    physics::DragFunction,
    simulation::Simulation,
    units::{
        acceleration, length, length::meter, typenum::P2, velocity, Acceleration, Angle, ConstZero,
        Energy, Length, Ratio, Time, Velocity,
    },
    vectors::{MyVector3, Norm},
};

pub trait Measurements {
    fn time(&self) -> Time;
    fn acceleration(&self) -> Acceleration;
    fn velocity(&self) -> Velocity;
    fn mach(&self) -> Ratio;
    fn energy(&self) -> Energy;
    fn distance(&self) -> Length;
    fn elevation(&self) -> Length;
    fn windage(&self) -> Length;
    fn angle(&self) -> Angle;
    fn vertical_angle(&self) -> Angle;
    fn horizontal_angle(&self) -> Angle;
    fn position(&self) -> MyVector3<length::Dimension>;
    fn offset_vertical_angle(&self, offset: Length) -> Angle;
    fn offset_horizontal_angle(&self, offset: Length) -> Angle;
    fn lerp(&self, other: &Self, x: Length) -> Self;
}

// Output of iteration, need a better name to encapsulate a moving projectile
#[derive(Debug)]
pub struct Packet<'a, D> {
    pub(crate) simulation: &'a Simulation<D>, //Simulation this came from, used for various calculations
    pub(crate) time: Time,                    // Position in time (s)
    pub(crate) position: MyVector3<length::Dimension>, // Position (m)
    pub(crate) velocity: MyVector3<velocity::Dimension>, // Velocity (m/s)
    pub(crate) acceleration: MyVector3<acceleration::Dimension>, // Acceleration (m/s^2)
}

impl<D> Measurements for Packet<'_, D>
where
    D: DragFunction,
{
    fn time(&self) -> Time {
        self.time
    }

    fn acceleration(&self) -> Acceleration {
        self.acceleration.norm()
    }

    fn velocity(&self) -> Velocity {
        let velocity = self.simulation.velocity() + self.velocity;
        velocity.norm()
    }

    // During the simulation, the velocity of the projectile is rotated to allign with
    // the shooter's bearing (azimuth and line of sight)
    // This function returns the position rotated back to the initial frame of reference
    // This is used during zero'ing and is output in the drop table
    fn position(&self) -> MyVector3<length::Dimension> {
        let position = self.simulation.position() + self.position;
        position
            .pivot_y(-self.simulation.shooter.yaw())
            .pivot_z(-self.simulation.shooter.pitch())
            .pivot_x(-self.simulation.shooter.roll())
    }

    fn mach(&self) -> Ratio {
        let velocity = self.simulation.velocity() + self.velocity - self.simulation.wind_velocity();
        let velocity = velocity.norm();
        self.simulation.mach(velocity)
    }

    fn energy(&self) -> Energy {
        self.velocity().powi(P2::new()) * self.simulation.projectile.weight * 0.5
    }

    // Positions relative to line of sight (shooter_pitch)
    fn distance(&self) -> Length {
        self.position().get_x()
    }

    fn elevation(&self) -> Length {
        self.position().get_y()
    }

    fn windage(&self) -> Length {
        self.position().get_z()
    }

    fn angle(&self) -> Angle {
        let compare = MyVector3::new(Length::new::<meter>(1.0), Length::ZERO, Length::ZERO);
        self.position().angle(&compare)
    }

    fn vertical_angle(&self) -> Angle {
        self.offset_vertical_angle(Length::ZERO)
    }

    fn horizontal_angle(&self) -> Angle {
        self.offset_horizontal_angle(Length::ZERO)
    }

    // This gives adjustment - opposite sign relative to desired offset
    // Always done in meters for now, due to relative_position()
    fn offset_vertical_angle(&self, offset: Length) -> Angle {
        let sign = if self.elevation() >= offset {
            1.0
        } else {
            -1.0
        };

        let position = MyVector3::new(self.distance(), self.elevation(), Length::ZERO);
        let desired = MyVector3::new(self.distance(), offset, Length::ZERO);

        position.angle(&desired) * sign
    }

    // This gives adjustment - opposite sign relative to desired offset
    // Always done in meters for now, due to relative_position()
    fn offset_horizontal_angle(&self, offset: Length) -> Angle {
        let sign = if self.windage() >= offset { 1.0 } else { -1.0 };

        let position = MyVector3::new(self.distance(), Length::ZERO, self.windage());
        let desired = MyVector3::new(self.distance(), Length::ZERO, offset);

        position.angle(&desired) * sign
    }

    fn lerp(&self, other: &Self, x: Length) -> Self {
        let dp = other.position - self.position;
        let dv = other.velocity - self.velocity;
        let da = other.acceleration - self.acceleration;
        let dt = other.time - self.time;

        let dx = x - self.distance();
        let slope = dx / dp.get_x();

        let position = self.position + dp * slope;
        let velocity = self.velocity + dv * slope;
        let acceleration = self.acceleration + da * slope;
        let time = self.time + dt * slope;

        Self {
            simulation: self.simulation,
            time,
            position,
            velocity,
            acceleration,
        }
    }
}
